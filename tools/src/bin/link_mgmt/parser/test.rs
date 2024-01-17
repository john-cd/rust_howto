pub fn get_test_markdown() -> String {
    let md: &'static str = "
 <http://url0/>

 [text1](url1)

 [text2][lbl2]

 [lbl2]: url2 \"title2\"

 [text3][]

 [text3]: url3

 [text4]

 [text4]: url4

 [![image_text5](image_url5)](url5)

 [image_lbl5]: image_url5 \"title5\"

 [![image7][image_lbl7]][lbl7]

 [image_lbl7]: image_url7

 [lbl7]: url7

 [![image_lbl8]][lbl8]

 [image_lbl8]: image_url8

 [lbl8]: url8

 [![image_lbl9][]][lbl9]

 [image_lbl9]: image_url9

 [lbl9]: url9

 [text10][missing_lbl10]
";
    md.to_owned()
}
