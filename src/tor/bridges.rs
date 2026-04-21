pub fn default_obfs4_bridges() -> &'static [&'static str] {
    &[
        "obfs4 192.95.36.142:443 CDF2E852BF539B82BD10E27E9115A31734E378C2 cert=qUVQ0srL1JI/vO6V6m/24anYXiJD3QP2HgzUKQtQ7GRqqUvs7P+tG43RtAqdhLOALP7DJQ iat-mode=1",
        "obfs4 37.218.245.14:38224 D9A82D2F9C2F65A18407B1D2B764F130847F8B5D cert=bjRjMferJTonT3jmdtzx/nCTBE7XRcrz3g=1 iat-mode=0",
        "obfs4 85.31.186.98:443 011F2599C0E9B27EE74B353155E244813763C3E5 cert=ayq0XzCwhpdysn5o0EyDUbmSOx3X/oTEbzDMvczHOdBJKlvIdHHLJGkZARtT4dcBFArPPg iat-mode=0",
        "obfs4 85.31.186.26:443 91A6354697E6B02A386312F68D82CF86824D3606 cert=PBwr+S8JTVZo6MPdHnkTwXJPILWADLqfMGoVvhZClMq/Urndyd42BwX9YFJHZnBB3H0XCw iat-mode=0",
        "obfs4 144.217.20.138:80 FB70B257C162BF1038CA669D568D76F5B7F0BABB cert=vYIV5MgrghGQvZPIi1tJwnzorMgqgmlKaB77Y3Z9Q/v94wZBOAXkW+fdx4aSxLVnKO+xNw iat-mode=0",
        "obfs4 146.57.248.225:22 10A6CD36A537FCE513A322361547444B393989F0 cert=K1gDtDAIcUfeLqbstggjIw2rtgIKqdIhUlHp82XRqNSq/mtAjp1BIC9vHKJ2FAEpGssTPw iat-mode=0",
    ]
}


pub fn default_snowflake_bridge() -> &'static str {
    "snowflake 192.0.2.3:80 2B280B23E1107BB62ABFC40DDCC8824814F80A72 url=https://snowflake-broker.torproject.net/ fronts=foursquare.com,github.githubassets.com ice=stun:stun.l.google.com:19302,stun:stun.antisip.com:3478,stun:stun.bluesip.net:3478,stun:stun.dus.net:3478,stun:stun.epygi.com:3478,stun:stun.sonetel.com:3478,stun:stun.uls.co.za:3478,stun:stun.voipgate.com:3478,stun:stun.voys.nl:3478 utls-imitate=hellorandomizedalpn"
}


pub fn default_webtunnel_bridges() -> &'static [&'static str] {
    &[
        "webtunnel [2001:db8:b2c0:0:a200::1]:443 A0DFEE5EE77ED72BC245D18A3F52F9F92E9E8E0C url=https://d3pyjtpvxs6z0u.cloudfront.net/C7z1W5sr ver=0.0.1",
        "webtunnel [2001:db8:f0d4:0:9c00::1]:443 5AE1EA34DFA5D14D23F0C8A9A0F0AA0A7A69B9C2 url=https://cdn.example.net/9cB4kL2m ver=0.0.1",
    ]
}


pub fn default_conjure_bridge() -> &'static str {
    "conjure 192.0.2.3:80 FC637DD4AABFDE7DB56DA0F1B52D16B7B09B64A3 url=https://registration.refraction.network/api/register-bidirectional"
}
