use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}


impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method.into();
                parsed_version = version.into();
                parsed_resource = resource.into();
            } else if line.contains(":") {
                let (k, v) = process_header_line(line);
                parsed_headers.insert(k, v);
            } else if line.len() == 0 {} else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.parse().unwrap(),
        }
    }
}

fn process_req_line(str: &str) -> (Method, Resource, Version) {
    let mut words = str.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into()
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut headers = s.split(":");
    (
        headers.next().unwrap().trim().to_string(),
        headers.next().unwrap().trim().parse().unwrap()
    )
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::httprequest::{HttpRequest, Method, process_header_line, Resource, Version};

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_process_header_line() {
        let s = "k1:v1";
        assert_eq!((String::from("k1"), String::from("v1")), process_header_line(s))
    }

    #[test]
    fn test_read_http() {
        let s = String::from("GET /test HTTP/1.1\r\nHost:localhost\r\n\n");
        let req = HttpRequest::from(s);
        assert_eq!(
            HttpRequest {
                method: Method::Get,
                version: Version::V1_1,
                resource: Resource::Path(String::from("/test")),
                headers: HashMap::from([(String::from("Host"), String::from("localhost"))]),
                msg_body: String::from("")
            },
            req
        )
    }
}