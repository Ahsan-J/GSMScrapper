use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom};
use regex::bytes::Regex;
use reqwest;
use std::{rc::Rc, slice::Iter};
pub mod model;

pub fn get_element_by_attribute_value(
    childrens: &Iter<Rc<Node>>,
    attribute_key: &String,
    attribute_value: &String,
) -> Option<Rc<Node>> {
    let children_clone = childrens.clone();

    for child in children_clone {
        match child.data {
            NodeData::Element { ref attrs, .. } => {
                let attributes = attrs.borrow();
                let attribute = attributes.iter().find(|&a| {
                    a.name.local.to_string() == attribute_key.to_string()
                        && a.value.to_string() == attribute_value.to_string()
                });

                // id attribute found with id
                if attribute.is_some() {
                    return Some(child.clone());
                }
            }
            _ => {}
        }

        let next_childrens = child.children.borrow();

        if next_childrens.len() > 0 {
            let data = get_element_by_attribute_value(
                &next_childrens.iter(),
                attribute_key,
                attribute_value,
            );
            if data.is_some() {
                return Some(data.unwrap());
            }
        }
    }
    None
}

pub fn get_text_value(element: &Rc<Node>) -> String {
    match element.data {
        NodeData::Text { ref contents, .. } => contents.borrow().to_string(),
        _ => String::from(""),
    }
}

pub fn get_mobile_info(node: &Handle) -> model::MobileData {
    let mut mobile_data = model::MobileData {
        url: String::from(""),
        title: String::from(""),
        os: String::from(""),
        size: String::from(""),
        cpu: String::from(""),
        gpu: String::from(""),
        has_fingerprint: false,
        has_nfc: false,
        has_dual_sim: false,
        back_camera: Vec::new(),
        front_camera: String::from(""),
        card_slot: String::from(""),
        usb: String::from(""),
        battery: String::from(""),
        storage: Vec::new(),
        chipset: String::from(""),
        price: 0,
        quantity: 0,
    };

    for attribute in model::get_gsm_attributes() {
        mobile_data.define_value(
            &attribute,
            &get_text_value(
                get_element_by_attribute_value(
                    &node.children.borrow().iter(),
                    &"data-spec".to_string(),
                    &attribute,
                )
                .unwrap()
                .children
                .take()
                .last()
                .unwrap(),
            ),
        );
    }

    return mobile_data;
}

pub async fn process_gsm_url(url: &String) -> model::MobileData {
    let url_reg = Regex::new(r"(https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b(?:[-a-zA-Z0-9@:%_\+.~#?&//=]*))").unwrap();

    if !url_reg.is_match(url.as_bytes()) {
        // Exit program if the regex is not a valid string
        panic!("Regex not matching proper url pattern")
    }

    let body = reqwest::get(url)
        .await
        .expect("Cannot make a request")
        .text()
        .await
        .expect("Cannot convert body text");

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut body.as_bytes())
        .unwrap();

    return get_mobile_info(&dom.document);
}