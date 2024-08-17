extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_http("HTTP/2.0");
    TcpServer::set_map_code_page(vec![(
        String::from("404 NOT FOUND"),
        Response::new_from_file("examples_rs/defpage.html", "text/html"),
    )]);

    let server = TcpServer::new(Server::get_server("127.0.0.1:443"), ThreadPool::new(4));

    Server::launch_range_port(server, 80..81);
}

struct Server;

impl SeverControl for Server {
    #[inline]
    fn match_methods(request: &Request, response: &mut Response) {
        match request.metod.as_str() {
            "GET" => Self::match_get(request, response),
            "POST" => Self::match_post(request, response),
            "PUT" => Self::match_put(request, response),
            _ => {}
        }
    }

    #[inline]
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener {
        TcpListener::bind(ip_addr).unwrap()
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.url.as_str() {
            // Work)
            "/qwe" => response.html(
                |resp| {
                    resp.echo(r#"<meta charset="utf-8">"#);
                    resp.echo(r#"<title>Cat Dark Net</title>"#);
                },
                |resp| {
                    resp.echo("<h1>123</h1>");
                    resp.echo("<h3>123</h3>");
                    resp.echo("<p>123</p>");
                },
            ),

            // Work Only Cookie( Just an example.
            "/response" => {
                response.set_file("examples_rs/webpage.html", "text/html");
                response.cookie.add("net", "qwe");
                response.cookie.delete("qwe");
            }

            // Don't Work( Just an example.
            "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
            // Don't Work( Just an example.
            "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
            // Don't Work( Just an example.
            "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
            // Don't Work( Just an example.
            "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
            // Don't Work( Just an example.
            "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),

            // Work)
            "/wer" => response.set_redirect("/response"),
            // Work)
            "/sleep" => std::thread::sleep(std::time::Duration::from_secs(30)),
            _ => {}
        }
    }

    #[inline]
    fn match_post(_request: &Request, _response: &mut Response) {}
    #[inline]
    fn match_put(_request: &Request, _response: &mut Response) {}
}
