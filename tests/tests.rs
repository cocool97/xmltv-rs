#[cfg(test)]
mod tests {
    use xmltv_rs::{XMLTVChannel, XMLTVChannelDisplayName, XMLTVProgram, XMLTVProgramTitle, XMLTV};

    #[test]
    fn test_empty_xmltv() {
        let xmltv = XMLTV::new();
        let mut writer: Vec<u8> = Vec::new();
        xmltv.build(&mut writer).unwrap();

        let expected = "<tv />\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    #[test]
    fn test_generator_info_empty_xmltv() {
        let mut xmltv = XMLTV::new();
        xmltv.add_generator_info_name("xmltv-rs".into());

        let mut writer: Vec<u8> = Vec::new();
        xmltv.build(&mut writer).unwrap();

        let expected = "<tv generator-info-name=\"xmltv-rs\" />\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    #[test]
    pub fn test_xmltv_one_channel() {
        let mut xmltv = XMLTV::new();
        xmltv.add_generator_info_name("xmltv-rs".into());

        let display_name = XMLTVChannelDisplayName::new("TF1".into(), None);

        let channel = XMLTVChannel::new("TF1".into(), "1".into(), display_name);
        xmltv.add_channel(channel).unwrap();

        let mut writer: Vec<u8> = Vec::new();
        xmltv.build(&mut writer).unwrap();

        let expected = "<tv generator-info-name=\"xmltv-rs\">
\t<channel channel=\"TF1\" id=\"1\">
\t\t<display-name>TF1</display-name>
\t</channel>
</tv>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    #[test]
    pub fn test_xmltv_programs() {
        let mut xmltv = XMLTV::new();

        let programs = vec![
            (
                "1",
                "Les feux de l'amour",
                None,
                "2021-10-09 12:00:00 +0200",
                "2021-10-09 13:00:00 +0200",
            ),
            (
                "2",
                "Le journal",
                Some("fr-FR"),
                "2021-10-09 12:20:00 +0200",
                "2021-10-09 12:35:00 +0200",
            ),
            (
                "3",
                "Le journal",
                None,
                "2021-10-09 13:00:00 +0200",
                "2021-10-09 13:40:00 +0200",
            ),
        ];

        for program in programs {
            let title = XMLTVProgramTitle {
                title: program.1.into(),
                lang: program.2.map(|v| v.into()),
            };
            let mut xmltv_program = XMLTVProgram::new(program.0.into(), program.3.into(), title);
            xmltv_program.add_stop_date(program.4.into());

            xmltv.add_program(xmltv_program).unwrap();
        }

        let mut writer: Vec<u8> = Vec::new();
        xmltv.build(&mut writer).unwrap();

        let expected = "<tv>
\t<programme channel=\"1\" start=\"2021-10-09 12:00:00 +0200\" stop=\"2021-10-09 13:00:00 +0200\">
\t\t<title>Les feux de l'amour</title>
\t</programme>
\t<programme channel=\"2\" start=\"2021-10-09 12:20:00 +0200\" stop=\"2021-10-09 12:35:00 +0200\">
\t\t<title lang=\"fr-FR\">Le journal</title>
\t</programme>
\t<programme channel=\"3\" start=\"2021-10-09 13:00:00 +0200\" stop=\"2021-10-09 13:40:00 +0200\">
\t\t<title>Le journal</title>
\t</programme>
</tv>\n";

        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    #[test]
    pub fn test_complex_xmltv() {
        let mut xmltv = XMLTV::new();
        xmltv.add_generator_info_name("xmltv-rs".into());

        let channels = vec![
            ("TF1", "1", Some("https://tf1.fr"), None),
            ("France 2", "2", None, Some("http://icon-france-2.fr")),
            (
                "France 3",
                "3",
                Some("https://www.france.tv/france-3"),
                None,
            ),
        ];

        for elem in channels {
            let display_name = XMLTVChannelDisplayName::new(elem.0.into(), None);

            let mut channel = XMLTVChannel::new(elem.0.into(), elem.1.into(), display_name);
            if let Some(url) = elem.2 {
                channel.add_url(url.into());
            }

            if let Some(icon) = elem.3 {
                channel.add_icon(icon.into());
            }

            xmltv.add_channel(channel).unwrap();
        }

        let mut writer: Vec<u8> = Vec::new();
        xmltv.build(&mut writer).unwrap();

        let expected = "<tv generator-info-name=\"xmltv-rs\">
\t<channel channel=\"TF1\" id=\"1\">
\t\t<display-name>TF1</display-name>
\t\t<url>https://tf1.fr</url>
\t</channel>
\t<channel channel=\"France 2\" id=\"2\">
\t\t<display-name>France 2</display-name>
\t\t<icon src=\"http://icon-france-2.fr\" />
\t</channel>
\t<channel channel=\"France 3\" id=\"3\">
\t\t<display-name>France 3</display-name>
\t\t<url>https://www.france.tv/france-3</url>
\t</channel>
</tv>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }
}
