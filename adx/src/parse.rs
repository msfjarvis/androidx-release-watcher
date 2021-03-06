use crate::{channel::Channel, package::MavenPackage};
use roxmltree::{Document, NodeType};
use semver::Version;
use std::convert::TryFrom;
use std::result::Result;

/// Downloads the Maven master index for Google's Maven Repository
/// and returns the XML as a String
async fn get_maven_index() -> surf::Result<String> {
    Ok(
        surf::get("https://dl.google.com/dl/android/maven2/master-index.xml")
            .recv_string()
            .await?,
    )
}

/// Downloads the group index for the given group.
async fn get_group_index(group: &str) -> surf::Result<String> {
    Ok(surf::get(format!(
        "https://dl.google.com/dl/android/maven2/{}/group-index.xml",
        group.replace(".", "/")
    ))
    .recv_string()
    .await?)
}

/// Parses a given master-index.xml and filters the found packages based on
// `search_term`.
fn filter_groups(doc: Document, search_term: &str) -> Vec<String> {
    let mut groups = vec![];
    for i in doc.descendants() {
        let tag = i.tag_name().name();
        if tag.contains(search_term) {
            groups.push(tag.to_string());
        }
    }
    groups
}

/// Given a list of groups, returns a `Vec<MavenPackage>` of all artifacts.
async fn parse_packages(groups: Vec<String>, channel: Channel) -> surf::Result<Vec<MavenPackage>> {
    let mut packages = Vec::new();
    for group_name in groups.iter() {
        let group_index = get_group_index(group_name).await?;
        let doc = Document::parse(&group_index)?;
        let mut is_next_root = false;
        let mut group = "";
        doc.descendants().for_each(|node| match node.node_type() {
            NodeType::Root => is_next_root = true,
            NodeType::Element => {
                if is_next_root {
                    group = node.tag_name().name();
                    is_next_root = false;
                } else if !group.is_empty() {
                    let mut versions: Vec<Version> = node
                        .attribute("versions")
                        .unwrap()
                        .split(',')
                        .map(|v| Version::parse(v))
                        // Only take values that were correctly parsed
                        .take_while(|x| x.is_ok())
                        // Unwrap values that were previously determined to be safe
                        .map(|x| x.unwrap())
                        .collect();
                    versions.retain(|x| {
                        if let Ok(c) = Channel::try_from(x.to_owned()) {
                            c >= channel
                        } else {
                            false
                        }
                    });
                    if !versions.is_empty() {
                        versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
                        packages.push(MavenPackage {
                            group_id: String::from(group),
                            artifact_id: node.tag_name().name().to_string(),
                            latest_version: versions.get(0).unwrap().to_string(),
                        })
                    }
                }
            }
            _ => (),
        });
    }
    Ok(packages)
}

pub(crate) async fn parse(
    search_term: &str,
    channel: Channel,
) -> Result<Vec<MavenPackage>, Box<dyn std::error::Error + 'static>> {
    let maven_index = get_maven_index().await?;
    let doc = Document::parse(&maven_index)?;
    let groups = filter_groups(doc, search_term);
    Ok(parse_packages(groups, channel).await?)
}
