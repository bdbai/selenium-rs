use webdriver::Browser;

// Session Request Object
#[derive(Serialize, Deserialize)]
pub struct NewSessionRequest {
    #[serde(rename = "desiredCapabilities")]
    desired_capabilities: DesiredCapabilitiesRequest,
}



#[derive(Serialize, Deserialize, Default)]
pub struct ChromeOptions{
    pub args: Vec<String>,
//    binary: String,
}

#[derive(Serialize, Deserialize)]
pub struct DesiredCapabilitiesRequest {
    #[serde(rename = "browserName")]
    pub browser_name: Browser,
    pub chromeOptions: ChromeOptions
}

impl NewSessionRequest {
    pub fn new(browser_name: &str, options: DesiredCapabilitiesRequest) -> NewSessionRequest {
        NewSessionRequest {
            desired_capabilities: options,
        }
    }
}



impl DesiredCapabilitiesRequest {
    pub fn create(browser: Browser) -> DesiredCapabilitiesRequest {
        DesiredCapabilitiesRequest {
            browser_name: browser,
            chromeOptions: ChromeOptions {
                args: vec!["--headless".to_string(), "--disable-gpu".to_string()],
//                binary: "/usr/bin/google-chrome".to_string()
            }
        }
    }
}

// Session Response object
#[derive(Serialize, Deserialize)]
pub struct NewSessionResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
}

impl NewSessionResponse {
    pub fn get_session_id(self) -> String {
        self.session_id
    }
}

#[derive(Deserialize)]
pub struct ScreenshotResponse {
    pub value: String,
}


// Title Response Object
#[derive(Deserialize)]
pub struct TitleResponse {
    value: String,
}

impl TitleResponse {
    pub fn get_title(self) -> String {
        self.value
    }
}
