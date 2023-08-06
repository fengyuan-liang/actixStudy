use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}


impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let r = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &r.version(),
            &r.status_code(),
            &r.status_text(),
            &r.headers(),
            &res.body.unwrap().len(),
            &r.body(),
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let h = HashMap::from([("Content-Type", "text/html")]);
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), ()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }


    pub fn version(&self) -> &'a str {
        self.version
    }
    pub fn status_code(&self) -> &'a str {
        self.status_code
    }
    pub fn status_text(&self) -> &'a str {
        self.status_text
    }
    pub fn headers(&self) -> String {
        let mut header_string: String = "".into();
        for (k, v) in self.headers.clone().unwrap() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => { b.as_str() },
            None => "",
        }
    }

    pub fn set_version(&mut self, version: &'a str) {
        self.version = version;
    }
    pub fn set_status_code(&mut self, status_code: &'a str) {
        self.status_code = status_code;
    }
    pub fn set_status_text(&mut self, status_text: &'a str) {
        self.status_text = status_text;
    }
    pub fn set_headers(&mut self, headers: Option<HashMap<&'a str, &'a str>>) {
        self.headers = headers;
    }
    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::httpresponse::HttpResponse;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code:"200",
            status_text: "OK",
            headers: Some(HashMap::from([("Content-type", "text/html")])),
            body: Some("xxx".into())
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code:"404",
            status_text: "Not Found",
            headers: Some(HashMap::from([("Content-type", "text/html")])),
            body: Some("xxx".into())
        };
        let http_string: String = response_expected.into();
        assert_eq!(
            "HTTP/1.1 404 Not Found\r\nContent-type:text/html\r\nContent-Length: 3\r\n\r\nxxx",
            http_string
        )
    }
}