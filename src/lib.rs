use std::net;

pub struct Anybar {
    /// The UDP Port the Anybar is connected to
    pub port: u16,
    /// The last color that has been set
    pub color: Option<Color>,
}

pub enum Color {
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Black,
    Question,
    Exclamation,
}

impl Anybar {
    pub fn new(port: u16) -> Anybar {
        //TODO check if port is valid
        Anybar{port:port, color:None}
    }

    fn parse_color(color: Color) -> Vec<u8> {
        use Color::*;
        let col = match color {
            White       => "white",
            Red         => "red",
            Orange      => "orange",
            Yellow      => "yellow",
            Green       => "green",
            Cyan        => "cyan",
            Blue        => "blue",
            Purple      => "purple",
            Black       => "black",
            Question    => "question",
            Exclamation => "exclamation",
        };

        let mut parsed: Vec<u8> = Vec::new();
        parsed.extend(col.as_bytes()
                         .iter());
        parsed
    }

    fn socket(ip: &str, port: u16) -> net::UdpSocket {
        match net::UdpSocket::bind((ip, port)) {
            Ok(sock) => sock,
            Err(err) => panic!("Could not bind: {}", err),
        }
    }

    pub fn set_color(&self, color: Color) {
        let message = Anybar::parse_color(color);

        let socket = Anybar::socket("127.0.0.1", 0);
        let _ = socket.send_to(&message, ("127.0.0.1", self.port));
        drop(socket);
    }
}

impl Default for Anybar {
    fn default() -> Anybar {
        Anybar{port:1738, color:None}
    }
}
