use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlPropsController {
    pub key: String,
    pub width: i32,
    pub height: i32,
    pub smart: String,
    pub halign: String,
    pub valign: String,
    pub filename: String,
}

#[derive(Debug)]
pub struct Alignment {
    pub halign: String,
    pub valign: String,
    pub smart: bool,
}

#[derive(Debug)]
pub struct FlipImage {
    pub horizontal: bool,
    pub vertical: bool,
}

#[derive(Debug)]
pub struct UrlProps {
    pub width: i32,
    pub height: i32,
    pub filename: String,
    pub alignment: Alignment,
    pub flip: FlipImage,
}

pub fn build_url_props(uri: UrlPropsController) -> UrlProps {
    UrlProps {
        width: i32::abs(uri.width),
        height: i32::abs(uri.height),
        filename: uri.filename,
        alignment: Alignment {
            halign: if uri.halign == "" { "center".to_string() }  else { uri.halign.clone().replace("/", "") },
            valign:  if uri.valign == "" { "middle".to_string() } else { uri.valign.clone().replace("/", "") },
            smart: uri.smart.replace("/", "") == "smart" && (uri.halign == "" && uri.valign == ""),
        },
        flip: FlipImage {
            horizontal: uri.width < 0,
            vertical: uri.height < 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_build_url_props_with_detault_props() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 100,
            height: 200,
            smart: "".to_string(),
            halign: "".to_string(),
            valign: "".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 100);
        assert_eq!(url_props.height, 200);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "center");
        assert_eq!(url_props.alignment.valign, "middle");
        assert_eq!(url_props.alignment.smart, false);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }

    #[test]
    fn test_build_url_props_with_detault_props_and_other_sizes() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 600,
            height: 300,
            smart: "".to_string(),
            halign: "".to_string(),
            valign: "".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 600);
        assert_eq!(url_props.height, 300);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "center");
        assert_eq!(url_props.alignment.valign, "middle");
        assert_eq!(url_props.alignment.smart, false);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }

    #[test]
    fn test_build_url_props_with_smart() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 602,
            height: 303,
            smart: "/smart".to_string(),
            halign: "".to_string(),
            valign: "".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 602);
        assert_eq!(url_props.height, 303);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "center");
        assert_eq!(url_props.alignment.valign, "middle");
        assert_eq!(url_props.alignment.smart, true);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }

    #[test]
    fn test_build_url_props_with_smart_as_false_when_has_halign() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 602,
            height: 303,
            smart: "/smart".to_string(),
            halign: "/left".to_string(),
            valign: "".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 602);
        assert_eq!(url_props.height, 303);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "left");
        assert_eq!(url_props.alignment.valign, "middle");
        assert_eq!(url_props.alignment.smart, false);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }

    #[test]
    fn test_build_url_props_with_smart_as_false_when_has_valign() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 602,
            height: 303,
            smart: "/smart".to_string(),
            halign: "".to_string(),
            valign: "/top".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 602);
        assert_eq!(url_props.height, 303);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "center");
        assert_eq!(url_props.alignment.valign, "top");
        assert_eq!(url_props.alignment.smart, false);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }

    #[test]
    fn test_build_url_props_with_smart_as_false_when_has_halign_and_valign() {
        let url_props = build_url_props(UrlPropsController {
            key: "unsafe".to_string(),
            width: 602,
            height: 303,
            smart: "/smart".to_string(),
            halign: "/right".to_string(),
            valign: "/bottom".to_string(),
            filename: "image.jpg".to_string(),
        });

        assert_eq!(url_props.width, 602);
        assert_eq!(url_props.height, 303);
        assert_eq!(url_props.filename, "image.jpg");
        assert_eq!(url_props.alignment.halign, "right");
        assert_eq!(url_props.alignment.valign, "bottom");
        assert_eq!(url_props.alignment.smart, false);
        assert_eq!(url_props.flip.horizontal, false);
        assert_eq!(url_props.flip.vertical, false);
    }
}