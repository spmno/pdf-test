use pdfium_render::prelude::*;
use std::path::Path;

use llm_sdk::LlmSdk;
use llm_sdk::api::chat_completion::{ChatCompletionRequestBuilder, ChatCompletionMessage, UserMessage, SystemMessage};

#[tokio::main]
async fn main() {
    let file = "test.pdf";
    let result = read_pdf_info(&file, None).await;
    match result {
        Ok(()) => (),
        Err(err ) => {
            println!("err: {:?}", err);
        }
    }
}

async fn test_llm_result(text: &str) {
    let req = ChatCompletionRequestBuilder::default()
    .model("ep-20240929173453-xsp5x".to_string())
    .messages(vec![
        ChatCompletionMessage::System(SystemMessage {
            content: "你是一个保单助手，记录保单的内容，并根据问题进行回答".to_string(),
        }),
        ChatCompletionMessage::User(UserMessage {
            content: text.to_string(),
        }),
        ChatCompletionMessage::User(UserMessage {
            //content: "保单的生效区间是什么？".to_string(),
            content: "保单的赔付项目是什么？".to_string(),
            //content: "保险公司电话是多少？".to_string(),
            //content: "保障内容是什么？".to_string(),
        }),
    ])
    .build()
    .unwrap();
    let res = SDK.chat_completion(&req).await;
    match res {
        Ok(res) => {
            let choice = &res.choices[0];
            println!("result: {}", choice.message.content.as_ref().unwrap());
        },
        Err(err) => {
            println!("internal error: {:?}", err);
        }
    }

}

async fn read_pdf_info(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable.
    // See the "Dynamic linking" section below.

    let pdfium = Pdfium::default();

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... then render each page to a bitmap image, saving each image to a JPEG file.
    let mut total_len = 0;
    let mut total_content = "".to_string();
    for (index, page) in document.pages().iter().enumerate() {
        let text = page.text().unwrap();
        println!("index: {}, text: {}", index, text);
        println!("-------------------------------------------------------------------");
        total_len = total_len + text.len();
        total_content.push_str(&text.to_string());
    }
    println!("total len: {}", total_len);
    test_llm_result(&total_content).await;
    Ok(())
}



fn export_pdf_to_jpegs(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable.
    // See the "Dynamic linking" section below.

    let pdfium = Pdfium::default();

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .into_rgb8() // ... then converts it to an image::Image...
            .save_with_format(
                format!("test-page-{}.jpg", index), 
                image::ImageFormat::Jpeg
            ) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}

lazy_static::lazy_static! {
    static ref SDK: LlmSdk = LlmSdk::new(std::env::var("DOUBAO_API_KEY").unwrap());
}