# Giortes
![Docker](https://github.com/dimitrmo/giortes/actions/workflows/docker.yml/badge.svg)
![Rust](https://github.com/dimitrmo/giortes/actions/workflows/rust.yml/badge.svg)

## Endpoints

### Get version

#### Request

`GET /version`

    curl -i -H 'Accept: text/plain' http://localhost:8080/version

#### Response

    0.1.0

#### Get names

#### Request

`GET /giortes`

    curl -i -H 'Accept: application/json' http://localhost:8080/giortes

#### Response

    {
        "updated_at": 1650181654165,
        "copyright": "www.eortologio.gr",
        "endpoint": "https://www.giortes.gr/rss/si_av_me_el.xml",
        "names": [
            "Βάϊος, Βάϊα, Βάγια, Βαία, Γιούλη, Δάφνη, Δάφνης, Πάσχα Καθολικών",
            "Μεγάλη Δευτέρα, Πάγκαλος",
            "Μεγάλη Τρίτη"
        ]
    }
