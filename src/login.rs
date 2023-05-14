use std::any::Any;
use std::fmt::Display;
use std::time::Duration;
use fantoccini::{ClientBuilder, Locator};
use fantoccini::elements::Element;
use tokio::time::sleep;

pub struct Course {
    id: i32,
    title: String,
    kafedra: String,
    link: String,
    tasks: Option<Vec<Task>>,
}

pub struct Task {
    task_title: String,
    date: String,
    link: String,
}

pub async fn fun (username: String, password: String) -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");
    let  link = "https://msnp.knutd.edu.ua";
    c.goto(link).await?;

    c.find(Locator::Id("username"))
        .await?
        .send_keys(username.as_str())
        .await?;
    c.find(Locator::Id("password"))
        .await?
        .send_keys(password.as_str())
        .await?;
    c.find(Locator::Id("loginbtn"))
        .await?
        .click()
        .await?;
    c.goto(&format!("{link}/my")).await?;
    sleep(Duration::from_millis(2000)).await;

    /*------------------------------*/

    let dashboard_deck = c.find(Locator::Css("div.card-deck.dashboard-card-deck"))
        .await?;
    let dashboard_cards = dashboard_deck.find_all(Locator::Css("div.card.dashboard-card"))
        .await?;

    let mut data: Vec<Course> = Vec::new();
    let mut i: i32 = 0;
    for card in dashboard_cards {
        i += 1;
        let course_name = card.find(Locator::Css("span.multiline"))
            .await?;
        let kaf_name = card.find(Locator::Css("span.categoryname"))
            .await?;
        let link_res = card.find(Locator::Css("a"))
            .await?;
        let link_href = link_res.attr("href")
            .await?
            .unwrap_or_default();
        sleep(Duration::from_millis(1000)).await;

        let obj = Course {
            id: i,
            title: course_name.text().await?,
            kafedra: kaf_name.text().await?,
            link: link_href,
            tasks: None,
        };
        &data.push(obj);
    }

    let section_timeline = c.find(Locator::Css("section.block_timeline"))
        .await?;
    section_timeline.find(Locator::Css("button.btn-outline-secondary.dropdown-toggle.icon-no-margin"))
        .await?
        .click()
        .await?;
    section_timeline.find(Locator::Css("a[aria-label='Вибрати все параметр фільтра']"))
        .await?
        .click()
        .await?;
    sleep(Duration::from_millis(1000)).await;

    let task_bar = section_timeline.find(Locator::Css("div[data-region='paged-content-page']"))
        .await?;
    let dates = task_bar.find_all(Locator::Css(".h6.mt-3.mb-0"))
        .await?;
    let task_boxes = task_bar.find_all(Locator::Css("div.event-name-container"))
        .await?;

    let mut j: usize = 0;
    let mut data_tasks: Vec<Task> = Vec::new();
    for task_box in task_boxes {
        let task_a_tag = task_box.find(Locator::Css("a"))
            .await?;
        let task_title = task_a_tag.attr("aria-label")
            .await?
            .unwrap_or_default();
        let date_elem: &Element = &dates[j];
        let date_elem_text = date_elem
            .text()
            .await?;
        let task_link = task_a_tag.attr("href")
            .await?
            .unwrap_or_default();

        let obj = Task {
            task_title: task_title.to_string(),
            date: date_elem_text.to_string(),
            link: task_link.to_string(),
        };

        j += 1;
        &data_tasks.push(obj);
    }
    let line: &str = "------------------------";
    println!("\n***{0} Твої завдання в цьому місяці {1}***\n", line, line);
    for h in data_tasks {
        println!("{0}\n{1}\n{2}\n{3}", h.task_title, h.date, h.link, line);
    }
    println!("\n***{0} Твої курси {1}***\n", line, line);
    for k in data {
        println!("|{0}| {1}\n{2}\n{3}\n{4}", k.id, k.title, k.kafedra, k.link, line)
    }

    sleep(Duration::from_millis(2000)).await;
    use tokio::time::sleep;

    c.close().await?;

    Ok(())
}