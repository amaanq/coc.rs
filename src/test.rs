use tokio;

#[cfg(test)]
mod tests {
    use std::mem::transmute;
    use crate::http;

    #[test]
    fn test_player() {
        let client = http::Client::new(
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImU5Y2EyZTAwLWE5MzEtNDY3MC1iMjljLTZiMzcwNzEzN2I5ZiIsImlhdCI6MTY1NDc5OTA5NSwic3ViIjoiZGV2ZWxvcGVyLzE1ZTIxZTgwLTVlYWEtNTViNi01MTU1LWJlOWI2ZjY3Y2Y0NiIsInNjb3BlcyI6WyJjbGFzaCJdLCJsaW1pdHMiOlt7InRpZXIiOiJkZXZlbG9wZXIvc2lsdmVyIiwidHlwZSI6InRocm90dGxpbmcifSx7ImNpZHJzIjpbIjgyLjguMjguNDAiXSwidHlwZSI6ImNsaWVudCJ9XX0.oUXQLFGcjMTklplXHicBL6FosrQU37T21SdB6MCJqJlYbvJp3zlneLkNTNuiXbPecEOQPPcBRu2twdVuphIemw"
                .to_string(),
        );

        let tag = "#2PP".to_string();
        match client.get_player(tag) {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(e) => match e {
                http::ApiError::Request(e) => {
                    println!("Request - {:?}", e);
                }
                http::ApiError::Api(e) => {
                    if e == reqwest::StatusCode::NOT_FOUND {
                        println!("Not found")
                    } else {
                        println!("Some other variant")
                    }
                }
            },
        }
    }

    #[test]
    fn test_clan(){
        let client = http::Client::new(
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImU5Y2EyZTAwLWE5MzEtNDY3MC1iMjljLTZiMzcwNzEzN2I5ZiIsImlhdCI6MTY1NDc5OTA5NSwic3ViIjoiZGV2ZWxvcGVyLzE1ZTIxZTgwLTVlYWEtNTViNi01MTU1LWJlOWI2ZjY3Y2Y0NiIsInNjb3BlcyI6WyJjbGFzaCJdLCJsaW1pdHMiOlt7InRpZXIiOiJkZXZlbG9wZXIvc2lsdmVyIiwidHlwZSI6InRocm90dGxpbmcifSx7ImNpZHJzIjpbIjgyLjguMjguNDAiXSwidHlwZSI6ImNsaWVudCJ9XX0.oUXQLFGcjMTklplXHicBL6FosrQU37T21SdB6MCJqJlYbvJp3zlneLkNTNuiXbPecEOQPPcBRu2twdVuphIemw"
                .to_string(),
        );
        let tag = "#2pp".to_string();

        let x = client.get_clan(tag).expect("Unable to get clan");

        println!("{:?}", x);
        assert!(true)
    }
}
