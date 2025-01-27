mod account;
mod commit;
mod identity;
mod message;

use message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider.");
    let mut connection = tungstenite::connect(
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post",
    )?
    .0;

    while connection.can_read() {
        let msg = connection.read()?;
        let decoded_msg = serde_json::from_str::<Message>(&msg.to_string());

        match decoded_msg {
            Ok(_msg) => {
                //println!("Received message: {:?}\n", msg);
            }
            Err(e) => {
                println!("Messag: {:?}", msg);
                println!("Error: {:?}\n", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use message::Message;

    #[test]
    fn test_deserialize_commit_message() {
        // Arrange
        let test = r#"{"did":"did:plc:a2f66gq7hjeo2ixej7jloq6l","time_us":1738009518762213,"kind":"commit","commit":{"rev":"3lgqrjyykpe2e","operation":"create","collection":"app.bsky.feed.post","rkey":"3lgqrjxvdr22a","record":{"$type":"app.bsky.feed.post","createdAt":"2025-01-27T20:25:16.812Z","embed":{"$type":"app.bsky.embed.external","external":{"description":"chillframe n boom booms","thumb":{"$type":"blob","ref":{"$link":"bafkreiexzs3lfglxebisqogqlfcd6heqadolsizh4d75cozuh4ho4gfkvu"},"mimeType":"image/jpeg","size":259009},"title":"Paracast - Twitch","uri":"https://www.twitch.tv/paracast"}},"facets":[{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"live"}],"index":{"byteEnd":11,"byteStart":6}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"WARFRAME"}],"index":{"byteEnd":29,"byteStart":20}},{"features":[{"$type":"app.bsky.richtext.facet#link","uri":"https://www.twitch.tv/paracast"}],"index":{"byteEnd":124,"byteStart":102}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"twitchtv"}],"index":{"byteEnd":135,"byteStart":126}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"streamer"}],"index":{"byteEnd":145,"byteStart":136}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"invaderzim"}],"index":{"byteEnd":157,"byteStart":146}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"alien"}],"index":{"byteEnd":164,"byteStart":158}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"invader"}],"index":{"byteEnd":173,"byteStart":165}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"gaming"}],"index":{"byteEnd":181,"byteStart":174}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"pcgaming"}],"index":{"byteEnd":191,"byteStart":182}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"twitch"}],"index":{"byteEnd":199,"byteStart":192}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"vtuber"}],"index":{"byteEnd":207,"byteStart":200}},{"features":[{"$type":"app.bsky.richtext.facet#tag","tag":"pngtuber"}],"index":{"byteEnd":217,"byteStart":208}}],"langs":["en"],"text":"going #live on some #WARFRAME come hang out and tell me what to do i r a noob doin missions! LETS GO!\nwww.twitch.tv/paracast\n\n#twitchtv #streamer #invaderzim #alien #invader #gaming #pcgaming #twitch #vtuber #pngtuber"},"cid":"bafyreih3ajklh4kf2qg5my33plciyxxaiap2anie53sw3gs3hpvpnsyeru"}}"#;

        // Act
        let msg: Message = serde_json::from_str(test).unwrap();

        // Assert
        assert_eq!(msg.did, "did:plc:a2f66gq7hjeo2ixej7jloq6l");
    }

    #[test]
    fn test_deserialize_identity_message() {
        // Arrange
        let test = r#"{"did":"did:plc:5gdnhg7ftde36ov4bplkyt3w","time_us":1738010191899082,"kind":"identity","identity":{"did":"did:plc:5gdnhg7ftde36ov4bplkyt3w","handle":"decormom.bsky.social","seq":4074993314,"time":"2025-01-27T20:36:31.590Z"}}"#;

        // Act
        let msg: Message = serde_json::from_str(test).unwrap();

        // Assert
        assert_eq!(msg.did, "did:plc:5gdnhg7ftde36ov4bplkyt3w");
    }

    #[test]
    fn test_deserialize_account_message() {
        // Arrange
        let test = r#"{"did":"did:plc:5gdnhg7ftde36ov4bplkyt3w","time_us":1738010191899082,"kind":"account","account":{"did":"did:plc:5gdnhg7ftde36ov4bplkyt3w","active":true,"seq":4074993314,"time":"2025-01-27T20:36:31.590Z"}}"#;

        // Act
        let msg: Message = serde_json::from_str(test).unwrap();

        // Assert
        assert_eq!(msg.did, "did:plc:5gdnhg7ftde36ov4bplkyt3w");
    }

    #[test]
    fn test_deserialize_media_message() {
        // Arrange
        let test = r#"{"did":"did:plc:qrztwc5uo3l2eukncjgdyado","time_us":1738013962258204,"kind":"commit","commit":{"rev":"3lgqvogletz2g","operation":"create","collection":"app.bsky.feed.post","rkey":"3lgqvofb47k23","record":{"$type":"app.bsky.feed.post","createdAt":"2025-01-27T21:39:20.009Z","embed":{"$type":"app.bsky.embed.recordWithMedia","media":{"$type":"app.bsky.embed.images","images":[{"alt":"","aspectRatio":{"height":200,"width":322},"image":{"$type":"blob","ref":{"$link":"bafkreidp3ibaalg3mz7ff2r2n34ekq5vcdolqsyqs2yfoavexzxdlbgtna"},"mimeType":"image/jpeg","size":143074}}]},"record":{"$type":"app.bsky.embed.record","record":{"cid":"bafyreiea2n6sqqwjmidt26ovf36peqg6kvvq5xm6vr5jl6im5iuzm34vti","uri":"at://did:plc:dgmiuoyozdy7yqb5eg45iqxd/app.bsky.feed.post/3lgpvprrink2k"}}},"langs":["en"],"text":"Marching off to work. Hi, ho, the MAGATs go!"},"cid":"bafyreibvm5gfy22kwtxoybb3ohxbxelmz3ia3zdswkdo63goyckjnt3twa"}}"#;

        // Act
        let result: Message = serde_json::from_str(test).unwrap();

        // Assert
        assert_eq!(result.did, "did:plc:qrztwc5uo3l2eukncjgdyado");
    }

    #[test]
    fn test_deserialize_commit_message_with_embed_video() {
        // Arrange
        let test = r#"{"did":"did:plc:wdudkvtfqtx4i43wwolw4nsi","time_us":1738013963610312,"kind":"commit","commit":{"rev":"3lgqvogx3fo2r","operation":"create","collection":"app.bsky.feed.post","rkey":"3lgqvohhfac2k","record":{"$type":"app.bsky.feed.post","createdAt":"2025-01-27T21:39:22.317Z","embed":{"$type":"app.bsky.embed.video","aspectRatio":{"height":480,"width":852},"video":{"$type":"blob","ref":{"$link":"bafkreicnu6vrfttjtkfaldecvpxnr6rhy4xkaplittpcce47xw75myc6zm"},"mimeType":"video/mp4","size":11275957}},"langs":["en"],"text":"We're back"},"cid":"bafyreidpnuvs2cnlzutkututtywmaoypwfbzrwccxvsvw3jukq3ukqre2u"}}"#;

        // Act
        let result: Message = serde_json::from_str(test).unwrap();

        // Assert
        assert_eq!(result.did, "did:plc:wdudkvtfqtx4i43wwolw4nsi");
    }
}
