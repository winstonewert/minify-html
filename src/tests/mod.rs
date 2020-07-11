#[cfg(test)]
fn _eval(src: &'static [u8], expected: &'static [u8], cfg: &super::Cfg) -> () {
    let mut code = src.to_vec();
    match super::hyperbuild_friendly_error(&mut code, cfg) {
        Ok(len) => {
            assert_eq!(std::str::from_utf8(&code[..len]).unwrap(), std::str::from_utf8(expected).unwrap());
        }
        Err(super::FriendlyError { code_context, message, .. }) => {
            println!("{}", message);
            println!("{}", code_context);
            assert!(false);
        }
    };
}

#[cfg(test)]
fn eval(src: &'static [u8], expected: &'static [u8]) -> () {
    _eval(src, expected, &super::Cfg {
        minify_js: false,
    });
}

#[cfg(test)]
fn eval_with_js_min(src: &'static [u8], expected: &'static [u8]) -> () {
    _eval(src, expected, &super::Cfg {
        minify_js: true,
    });
}

#[test]
fn test_collapse_whitespace() {
    eval(b"<a>   \n&#32;   </a>", b"<a> </a>");
}

#[test]
fn test_collapse_and_trim_whitespace() {
    eval(b"<label>   \n&#32;   </label>", b"<label></label>");
    eval(b"<label>   \n&#32;a   </label>", b"<label>a</label>");
    eval(b"<label>   \n&#32;a   b   </label>", b"<label>a b</label>");
}

#[test]
fn test_collapse_destroy_whole_and_trim_whitespace() {
    eval(b"<ul>   \n&#32;   </ul>", b"<ul></ul>");
    eval(b"<ul>   \n&#32;a   </ul>", b"<ul>a</ul>");
    eval(b"<ul>   \n&#32;a   b   </ul>", b"<ul>a b</ul>");
    eval(b"<ul>   \n&#32;a<pre></pre>   <pre></pre>b   </ul>", b"<ul>a<pre></pre><pre></pre>b</ul>");
}

#[test]
fn test_no_whitespace_minification() {
    eval(b"<pre>   \n&#32; \t   </pre>", b"<pre>   \n  \t   </pre>");
}

#[test]
fn test_self_closing_svg_tag_whitespace_removal() {
    eval(b"<svg><path d=a /></svg>", b"<svg><path d=a /></svg>");
    eval(b"<svg><path d=a/ /></svg>", b"<svg><path d=a/ /></svg>");
    eval(b"<svg><path d=\"a/\" /></svg>", b"<svg><path d=a/ /></svg>");
    eval(b"<svg><path d=\"a/\"/></svg>", b"<svg><path d=a/ /></svg>");
    eval(b"<svg><path d='a/' /></svg>", b"<svg><path d=a/ /></svg>");
    eval(b"<svg><path d='a/'/></svg>", b"<svg><path d=a/ /></svg>");
}

#[test]
fn test_removal_of_optional_tags() {
    eval(b"<ul><li>1</li><li>2</li><li>3</li></ul>", b"<ul><li>1<li>2<li>3</ul>");
    eval(b"<rt></rt>", b"<rt>");
    eval(b"<rt></rt><rp>1</rp><div></div>", b"<rt><rp>1</rp><div></div>");
    eval(b"<div><rt></rt></div>", b"<div><rt></div>");
}

#[test]
fn test_removal_of_optional_closing_p_tag() {
    eval(b"<p></p><address></address>", b"<p><address></address>");
    eval(b"<p></p>", b"<p>");
    eval(b"<map><p></p></map>", b"<map><p></p></map>");
    eval(b"<map><p></p><address></address></map>", b"<map><p><address></address></map>");
}

#[test]
fn test_attr_double_quoted_value_minification() {
    eval(b"<a b=\" hello \"></a>", b"<a b=\" hello \"></a>");
    eval(b"<a b=' hello '></a>", b"<a b=\" hello \"></a>");
    eval(b"<a b=&#x20;hello&#x20;></a>", b"<a b=\" hello \"></a>");
    eval(b"<a b=&#x20hello&#x20></a>", b"<a b=\" hello \"></a>");
}

#[test]
fn test_attr_single_quoted_value_minification() {
    eval(b"<a b=\"&quot;hello\"></a>", b"<a b='\"hello'></a>");
    eval(b"<a b='\"hello'></a>", b"<a b='\"hello'></a>");
    eval(b"<a b=&#x20;he&quotllo&#x20;></a>", b"<a b=' he\"llo '></a>");
}

#[test]
fn test_attr_unquoted_value_minification() {
    eval(b"<a b=\"hello\"></a>", b"<a b=hello></a>");
    eval(b"<a b='hello'></a>", b"<a b=hello></a>");
    eval(b"<a b=hello></a>", b"<a b=hello></a>");
}

#[test]
fn test_class_attr_value_minification() {
    eval(b"<a class=&#x20;c></a>", b"<a class=c></a>");
    eval(b"<a class=&#x20;c&#x20&#x20;d&#x20></a>", b"<a class=\"c d\"></a>");
    eval(b"<a class=&#x20&#x20&#x20;&#x20></a>", b"<a></a>");
    eval(b"<a class=\"  c\n \n  \"></a>", b"<a class=c></a>");
    eval(b"<a class=\"  c\n \nd  \"></a>", b"<a class=\"c d\"></a>");
    eval(b"<a class=\"  \n \n  \"></a>", b"<a></a>");
    eval(b"<a class='  c\n \n  '></a>", b"<a class=c></a>");
    eval(b"<a class='  c\n \nd  '></a>", b"<a class=\"c d\"></a>");
    eval(b"<a class='  \n \n  '></a>", b"<a></a>");
}

#[test]
fn test_d_attr_value_minification() {
    eval(b"<svg><path d=&#x20;c /></svg>", b"<svg><path d=c /></svg>");
    eval(b"<svg><path d=&#x20;c&#x20&#x20;d&#x20 /></svg>", b"<svg><path d=\"c d\"/></svg>");
    eval(b"<svg><path d=&#x20;&#x20&#x20&#x20 /></svg>", b"<svg><path/></svg>");
    eval(b"<svg><path d=\"  c\n \n  \" /></svg>", b"<svg><path d=c /></svg>");
    eval(b"<svg><path d=\"  c\n \nd  \" /></svg>", b"<svg><path d=\"c d\"/></svg>");
    eval(b"<svg><path d=\"  \n \n  \" /></svg>", b"<svg><path/></svg>");
    eval(b"<svg><path d='  c\n \n  ' /></svg>", b"<svg><path d=c /></svg>");
    eval(b"<svg><path d='  c\n \nd  ' /></svg>", b"<svg><path d=\"c d\"/></svg>");
    eval(b"<svg><path d='  \n \n  ' /></svg>", b"<svg><path/></svg>");
}

#[test]
fn test_boolean_attr_value_removal() {
    eval(b"<div hidden=\"true\"></div>", b"<div hidden></div>");
    eval(b"<div hidden=\"false\"></div>", b"<div hidden></div>");
    eval(b"<div hidden=\"1\"></div>", b"<div hidden></div>");
    eval(b"<div hidden=\"0\"></div>", b"<div hidden></div>");
    eval(b"<div hidden=\"abc\"></div>", b"<div hidden></div>");
    eval(b"<div hidden=\"\"></div>", b"<div hidden></div>");
    eval(b"<div hidden></div>", b"<div hidden></div>");
}

#[test]
fn test_empty_attr_removal() {
    eval(b"<div lang=\"  \"></div>", b"<div lang=\"  \"></div>");
    eval(b"<div lang=\"\"></div>", b"<div></div>");
    eval(b"<div lang=''></div>", b"<div></div>");
    eval(b"<div lang=></div>", b"<div></div>");
    eval(b"<div lang></div>", b"<div></div>");
}

#[test]
fn test_default_attr_value_removal() {
    eval(b"<a target=\"_self\"></a>", b"<a></a>");
    eval(b"<a target='_self'></a>", b"<a></a>");
    eval(b"<a target=_self></a>", b"<a></a>");
}

#[test]
fn test_script_type_attr_value_removal() {
    eval(b"<script type=\"application/ecmascript\"></script>", b"<script></script>");
    eval(b"<script type=\"application/javascript\"></script>", b"<script></script>");
    eval(b"<script type=\"text/jscript\"></script>", b"<script></script>");
}

#[test]
fn test_empty_attr_value_removal() {
    eval(b"<div a=\"  \"></div>", b"<div a=\"  \"></div>");
    eval(b"<div a=\"\"></div>", b"<div a></div>");
    eval(b"<div a=''></div>", b"<div a></div>");
    eval(b"<div a=></div>", b"<div a></div>");
    eval(b"<div a></div>", b"<div a></div>");
}

#[test]
fn test_space_between_attrs_minification() {
    eval(b"<div a=\" \" b=\" \"></div>", b"<div a=\" \"b=\" \"></div>");
    eval(b"<div a=' ' b=\" \"></div>", b"<div a=\" \"b=\" \"></div>");
    eval(b"<div a=&#x20 b=\" \"></div>", b"<div a=\" \"b=\" \"></div>");
    eval(b"<div a=\"1\" b=\" \"></div>", b"<div a=1 b=\" \"></div>");
    eval(b"<div a='1' b=\" \"></div>", b"<div a=1 b=\" \"></div>");
    eval(b"<div a=\"a\"b=\"b\"></div>", b"<div a=a b=b></div>");
}

#[test]
fn test_attr_value_backtick() {
    // The backtick is not interpreted as a quote; as such, the "b" attribute is interpreted as having an empty value,
    // and the "`hello`" attribute is a boolean attribute (also empty value).
    eval(b"<a b=`hello`></a>", b"<a b `hello`></a>");
}

#[test]
fn test_hexadecimal_entity_decoding() {
    eval(b"&#x30", b"0");
    eval(b"&#x0030", b"0");
    eval(b"&#x000000000000000000000000000000000000000000030", b"0");
    eval(b"&#x30;", b"0");
    eval(b"&#x0030;", b"0");
    eval(b"&#x000000000000000000000000000000000000000000030;", b"0");
    eval(b"&#x1151;", b"\xe1\x85\x91");
    eval(b"&#x11FFFF;", b"\xef\xbf\xbd");
    eval(b"&#xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;", b"\xef\xbf\xbd");
}

#[test]
fn test_decimal_entity_decoding() {
    eval(b"&#48", b"0");
    eval(b"&#0048", b"0");
    eval(b"&#000000000000000000000000000000000000000000048", b"0");
    eval(b"&#48;", b"0");
    eval(b"&#0048;", b"0");
    eval(b"&#000000000000000000000000000000000000000000048;", b"0");
    eval(b"&#4433;", b"\xe1\x85\x91");
    eval(b"&#1114112;", b"\xef\xbf\xbd");
    eval(b"&#999999999999999999999999999999999999999999999;", b"\xef\xbf\xbd");
}

#[test]
fn test_named_entity_decoding() {
    eval(b"&gt", b">");
    eval(b"&gt;", b">");
    eval(b"&amp", b"&");
    eval(b"&amp;", b"&");
    eval(b"&xxxyyyzzz", b"&xxxyyyzzz");
    eval(b"&ampere", b"&ere");
    eval(b"They & Co.", b"They & Co.");
    eval(b"if (this && that)", b"if (this && that)");
    // These entities decode to longer UTF-8 sequences, so we keep them encoded.
    eval(b"&nLt;", b"&nLt;");
    eval(b"&nLt;abc", b"&nLt;abc");
    eval(b"&nGt;", b"&nGt;");
}

#[test]
fn test_unintentional_entity_prevention() {
    eval(b"&ampamp", b"&ampamp");
    eval(b"&ampamp;", b"&ampamp;");
    eval(b"&amp;amp", b"&ampamp");
    eval(b"&amp;amp;", b"&ampamp;");
    eval(b"&&#97&#109;&#112;;", b"&ampamp;");
    eval(b"&&#97&#109;p;", b"&ampamp;");
    eval(b"&am&#112", b"&ampamp");
    eval(b"&am&#112;", b"&ampamp");
    eval(b"&am&#112&#59", b"&ampamp;");
    eval(b"&am&#112;;", b"&ampamp;");
    eval(b"&am&#112;&#59", b"&ampamp;");
    eval(b"&am&#112;&#59;", b"&ampamp;");

    eval(b"&l&#116", b"&amplt");
    eval(b"&&#108t", b"&amplt");
    eval(b"&&#108t;", b"&amplt;");
    eval(b"&&#108t&#59", b"&amplt;");
    eval(b"&amplt", b"&amplt");
    eval(b"&amplt;", b"&amplt;");

    eval(b"&am&am&#112", b"&am&ampamp");
    eval(b"&am&am&#112&#59", b"&am&ampamp;");

    eval(b"&amp&nLt;", b"&&nLt;");
    eval(b"&am&nLt;", b"&am&nLt;");
    eval(b"&am&nLt;a", b"&am&nLt;a");
    eval(b"&am&nLt", b"&am&nLt");
}

#[test]
fn test_left_chevron_entities_in_content() {
    eval(b"&LT", b"&LT");
    eval(b"&LT;", b"&LT");
    eval(b"&LT;;", b"&LT;;");
    eval(b"&LT;&#59", b"&LT;;");
    eval(b"&LT;&#59;", b"&LT;;");
    eval(b"&lt", b"&LT");
    eval(b"&lt;", b"&LT");
    eval(b"&lt;;", b"&LT;;");
    eval(b"&lt;&#59", b"&LT;;");
    eval(b"&lt;&#59;", b"&LT;;");
}

#[test]
fn test_comments_removal() {
    eval(b"<pre>a <!-- akd--sj\n <!-- \t\0f--ajk--df->lafj -->  b</pre>", b"<pre>a   b</pre>");
    eval(b"&a<!-- akd--sj\n <!-- \t\0f--ajk--df->lafj -->mp", b"&amp");
    eval(b"<script><!-- akd--sj\n <!-- \t\0f--ajk--df->lafj --></script>", b"<script><!-- akd--sj\n <!-- \t\0f--ajk--df->lafj --></script>");
}

#[test]
fn test_processing_instructions() {
    eval(b"<?php hello??? >>  ?>", b"<?php hello??? >>  ?>");
    eval(b"av<?xml 1.0 ?>g", b"av<?xml 1.0 ?>g");
}

#[cfg(feature = "js-esbuild")]
#[test]
fn test_js_minification() {
    eval_with_js_min(b"<script>let a = 1;</script>", b"<script>let a=1;</script>");
}
