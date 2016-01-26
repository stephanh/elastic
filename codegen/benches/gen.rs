#![feature(test)]
extern crate test;

use test::Bencher;

/*
	Thanks @DanielKeep for fixing this up, and providing the only useful implementations.
	
	Here we're testing options for building up a url efficiently.
	- The `format_url_macro` example is generated by `url_fmt_decl`
	- The `format_url_concat` example doesn't have a fn yet, but the parts are as follows:
		- Get url parts from `parse_path_parts`
		- Get url params from `parse_path_params`
		- Set a mutable `String` to the first part
		- Until there are no parts and params, emit a param, then a part
		- This will probably require a few helper fns, because will be difficult to express as a macro 
		  (parts are literals and params are Strings)

	The `format_url_concat` is slightly faster
*/

#[bench]
fn format_url_macro(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        format!("/{}/_alias/{}", index, name)
    });
}

#[bench]
fn format_url_concat(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        let mut url = "/".to_string();
        url = url + &index[..] + "/_alias/" + &name[..];
        url
    });
}

#[bench]
fn format_url_push(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        let mut url = String::with_capacity(1 + "/_alias/".len()
            + index.len() + name.len());
        url.push_str("/");
        url.push_str(&index);
        url.push_str("/_alias/");
        url.push_str(&name);
        url
    });
}

#[bench]
fn format_url_write(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        use std::fmt::Write;
        let mut url = String::with_capacity(1 + "/_alias/".len()
            + index.len() + name.len());
        write!(url, "/{}/_alias/{}", index, name).unwrap();
        url
    });
}