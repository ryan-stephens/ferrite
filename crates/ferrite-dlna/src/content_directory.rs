use anyhow::Result;
use ferrite_db::media_repo::{self, MediaItemRow};
use sqlx::SqlitePool;
use tracing::debug;

/// Parse a SOAP Browse request body and return the relevant parameters.
pub struct BrowseRequest {
    pub object_id: String,
    pub browse_flag: String,
    pub starting_index: u32,
    pub requested_count: u32,
}

/// Parse the SOAP XML body for a Browse action.
pub fn parse_browse_request(body: &str) -> Option<BrowseRequest> {
    let object_id = extract_xml_value(body, "ObjectID")?;
    let browse_flag =
        extract_xml_value(body, "BrowseFlag").unwrap_or_else(|| "BrowseDirectChildren".into());
    let starting_index = extract_xml_value(body, "StartingIndex")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let requested_count = extract_xml_value(body, "RequestedCount")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    Some(BrowseRequest {
        object_id,
        browse_flag,
        starting_index,
        requested_count,
    })
}

/// Handle a ContentDirectory Browse action.
/// Object ID "0" = root container, which lists all media items.
pub async fn handle_browse(
    pool: &SqlitePool,
    req: &BrowseRequest,
    http_base_url: &str,
) -> Result<String> {
    debug!(
        "DLNA Browse: object_id={}, flag={}, start={}, count={}",
        req.object_id, req.browse_flag, req.starting_index, req.requested_count
    );

    if req.browse_flag == "BrowseMetadata" && req.object_id == "0" {
        return Ok(wrap_browse_response(&root_container_didl(), 1, 1));
    }

    if req.object_id == "0" {
        // Browse root — list all media items
        let page = (req.starting_index / req.requested_count.max(1)) + 1;
        let items = media_repo::list_media_items(pool, None, page, req.requested_count).await?;
        let total = media_repo::count_media_items(pool, None).await? as u32;

        let didl = items_to_didl(&items, http_base_url);
        return Ok(wrap_browse_response(&didl, items.len() as u32, total));
    }

    // Browse a specific media item (BrowseMetadata)
    if let Some(item) = media_repo::get_media_item(pool, &req.object_id).await? {
        let didl = item_to_didl(&item, http_base_url);
        return Ok(wrap_browse_response(&didl, 1, 1));
    }

    // Unknown object — return empty
    Ok(wrap_browse_response("", 0, 0))
}

/// Handle GetSystemUpdateID — returns a static update ID.
pub fn handle_get_system_update_id() -> String {
    wrap_soap_response(
        "GetSystemUpdateID",
        "urn:schemas-upnp-org:service:ContentDirectory:1",
        "<Id>1</Id>",
    )
}

/// Handle GetSearchCapabilities — we don't support search.
pub fn handle_get_search_capabilities() -> String {
    wrap_soap_response(
        "GetSearchCapabilities",
        "urn:schemas-upnp-org:service:ContentDirectory:1",
        "<SearchCaps></SearchCaps>",
    )
}

/// Handle GetSortCapabilities.
pub fn handle_get_sort_capabilities() -> String {
    wrap_soap_response(
        "GetSortCapabilities",
        "urn:schemas-upnp-org:service:ContentDirectory:1",
        "<SortCaps></SortCaps>",
    )
}

/// Handle ConnectionManager GetProtocolInfo.
pub fn handle_get_protocol_info() -> String {
    let source_protocols = [
        "http-get:*:video/mp4:DLNA.ORG_PN=AVC_MP4_MP_SD_AAC_MULT5",
        "http-get:*:video/mp4:*",
        "http-get:*:video/x-matroska:*",
        "http-get:*:video/avi:*",
        "http-get:*:audio/mpeg:*",
        "http-get:*:audio/mp4:*",
        "http-get:*:audio/flac:*",
    ]
    .join(",");

    wrap_soap_response(
        "GetProtocolInfo",
        "urn:schemas-upnp-org:service:ConnectionManager:1",
        &format!(
            "<Source>{}</Source><Sink></Sink>",
            quick_xml::escape::escape(&source_protocols)
        ),
    )
}

/// Handle ConnectionManager GetCurrentConnectionIDs.
pub fn handle_get_current_connection_ids() -> String {
    wrap_soap_response(
        "GetCurrentConnectionIDs",
        "urn:schemas-upnp-org:service:ConnectionManager:1",
        "<ConnectionIDs>0</ConnectionIDs>",
    )
}

/// Handle ConnectionManager GetCurrentConnectionInfo.
pub fn handle_get_current_connection_info() -> String {
    wrap_soap_response(
        "GetCurrentConnectionInfo",
        "urn:schemas-upnp-org:service:ConnectionManager:1",
        "<RcsID>-1</RcsID>\
         <AVTransportID>-1</AVTransportID>\
         <ProtocolInfo></ProtocolInfo>\
         <PeerConnectionManager></PeerConnectionManager>\
         <PeerConnectionID>-1</PeerConnectionID>\
         <Direction>Output</Direction>\
         <Status>OK</Status>",
    )
}

// ---------------------------------------------------------------------------
// DIDL-Lite XML generation
// ---------------------------------------------------------------------------

fn root_container_didl() -> String {
    r#"<DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/">
<container id="0" parentID="-1" restricted="true" childCount="0">
<dc:title>Ferrite</dc:title>
<upnp:class>object.container.storageFolder</upnp:class>
</container>
</DIDL-Lite>"#
        .to_string()
}

fn items_to_didl(items: &[MediaItemRow], http_base_url: &str) -> String {
    let mut didl = String::from(
        r#"<DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/" xmlns:dlna="urn:schemas-dlna-org:metadata-1-0/">"#,
    );

    for item in items {
        didl.push_str(&item_to_didl_element(item, http_base_url));
    }

    didl.push_str("</DIDL-Lite>");
    didl
}

fn item_to_didl(item: &MediaItemRow, http_base_url: &str) -> String {
    format!(
        r#"<DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/" xmlns:dlna="urn:schemas-dlna-org:metadata-1-0/">{}</DIDL-Lite>"#,
        item_to_didl_element(item, http_base_url)
    )
}

fn item_to_didl_element(item: &MediaItemRow, http_base_url: &str) -> String {
    let title = quick_xml::escape::escape(item.title.as_deref().unwrap_or(&item.file_path));

    let mime = guess_mime_type(&item.file_path, &item.container_format);
    let upnp_class = if item.media_type == "audio" {
        "object.item.audioItem.musicTrack"
    } else {
        "object.item.videoItem"
    };

    let stream_url = format!("{}/api/stream/{}", http_base_url, item.id);

    let duration_str = item
        .duration_ms
        .map(|ms| format_dlna_duration(ms as u64))
        .unwrap_or_default();

    let resolution = match (item.width, item.height) {
        (Some(w), Some(h)) => format!("{}x{}", w, h),
        _ => String::new(),
    };

    let size_attr = if item.file_size > 0 {
        format!(r#" size="{}""#, item.file_size)
    } else {
        String::new()
    };

    let duration_attr = if !duration_str.is_empty() {
        format!(r#" duration="{}""#, duration_str)
    } else {
        String::new()
    };

    let resolution_attr = if !resolution.is_empty() {
        format!(r#" resolution="{}""#, resolution)
    } else {
        String::new()
    };

    format!(
        r#"<item id="{id}" parentID="0" restricted="true">
<dc:title>{title}</dc:title>
<upnp:class>{upnp_class}</upnp:class>
<res protocolInfo="http-get:*:{mime}:*"{size}{dur}{res}>{url}</res>
</item>"#,
        id = item.id,
        title = title,
        upnp_class = upnp_class,
        mime = mime,
        size = size_attr,
        dur = duration_attr,
        res = resolution_attr,
        url = quick_xml::escape::escape(&stream_url),
    )
}

// ---------------------------------------------------------------------------
// SOAP response wrappers
// ---------------------------------------------------------------------------

fn wrap_browse_response(didl_content: &str, number_returned: u32, total_matches: u32) -> String {
    let escaped_didl = quick_xml::escape::escape(didl_content);
    let body = format!(
        "<Result>{}</Result>\
         <NumberReturned>{}</NumberReturned>\
         <TotalMatches>{}</TotalMatches>\
         <UpdateID>1</UpdateID>",
        escaped_didl, number_returned, total_matches
    );
    wrap_soap_response(
        "Browse",
        "urn:schemas-upnp-org:service:ContentDirectory:1",
        &body,
    )
}

fn wrap_soap_response(action: &str, service_type: &str, body: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
<s:Body>
<u:{action}Response xmlns:u="{service_type}">
{body}
</u:{action}Response>
</s:Body>
</s:Envelope>"#,
        action = action,
        service_type = service_type,
        body = body,
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract a simple XML element value by tag name (naive but sufficient for SOAP).
fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);
    let start_idx = xml.find(&open)?;
    let after_open = &xml[start_idx + open.len()..];
    // Skip past the closing > of the opening tag
    let content_start = after_open.find('>')? + 1;
    let content = &after_open[content_start..];
    let end_idx = content.find(&close)?;
    Some(content[..end_idx].to_string())
}

/// Guess MIME type from file extension and container format.
fn guess_mime_type(file_path: &str, container_format: &Option<String>) -> &'static str {
    if let Some(fmt) = container_format.as_deref() {
        match fmt.to_lowercase().as_str() {
            "mp4" | "mov" => return "video/mp4",
            "matroska" | "webm" | "mkv" => return "video/x-matroska",
            "avi" => return "video/avi",
            "mp3" => return "audio/mpeg",
            "flac" => return "audio/flac",
            "ogg" => return "audio/ogg",
            _ => {}
        }
    }

    let ext = file_path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "mp4" | "m4v" => "video/mp4",
        "mkv" => "video/x-matroska",
        "avi" => "video/avi",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "mp3" => "audio/mpeg",
        "flac" => "audio/flac",
        "m4a" | "aac" => "audio/mp4",
        "ogg" | "opus" => "audio/ogg",
        _ => "video/mp4",
    }
}

/// Format duration in DLNA format: H:MM:SS.mmm
fn format_dlna_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = ms % 1000;
    format!("{}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_xml_value() {
        let xml = r#"<ObjectID>0</ObjectID>"#;
        assert_eq!(extract_xml_value(xml, "ObjectID"), Some("0".into()));
    }

    #[test]
    fn test_extract_xml_value_with_namespace() {
        let xml = r#"<u:Browse><ObjectID>root</ObjectID></u:Browse>"#;
        assert_eq!(extract_xml_value(xml, "ObjectID"), Some("root".into()));
    }

    #[test]
    fn test_format_dlna_duration() {
        assert_eq!(format_dlna_duration(0), "0:00:00.000");
        assert_eq!(format_dlna_duration(90_500), "0:01:30.500");
        assert_eq!(format_dlna_duration(7_200_000), "2:00:00.000");
    }

    #[test]
    fn test_guess_mime_type_from_container() {
        assert_eq!(
            guess_mime_type("test.mkv", &Some("matroska".into())),
            "video/x-matroska"
        );
        assert_eq!(
            guess_mime_type("test.mp4", &Some("mp4".into())),
            "video/mp4"
        );
    }

    #[test]
    fn test_guess_mime_type_from_extension() {
        assert_eq!(guess_mime_type("movie.mkv", &None), "video/x-matroska");
        assert_eq!(guess_mime_type("song.mp3", &None), "audio/mpeg");
        assert_eq!(guess_mime_type("video.avi", &None), "video/avi");
    }

    #[test]
    fn test_wrap_soap_response() {
        let resp = wrap_soap_response("Browse", "urn:test:service", "<Result>data</Result>");
        assert!(resp.contains("<u:BrowseResponse xmlns:u=\"urn:test:service\">"));
        assert!(resp.contains("<Result>data</Result>"));
        assert!(resp.contains("</u:BrowseResponse>"));
    }

    #[test]
    fn test_parse_browse_request() {
        let body = r#"<s:Envelope><s:Body><u:Browse>
            <ObjectID>0</ObjectID>
            <BrowseFlag>BrowseDirectChildren</BrowseFlag>
            <Filter>*</Filter>
            <StartingIndex>0</StartingIndex>
            <RequestedCount>50</RequestedCount>
            <SortCriteria></SortCriteria>
        </u:Browse></s:Body></s:Envelope>"#;

        let req = parse_browse_request(body).unwrap();
        assert_eq!(req.object_id, "0");
        assert_eq!(req.browse_flag, "BrowseDirectChildren");
        assert_eq!(req.starting_index, 0);
        assert_eq!(req.requested_count, 50);
    }
}
