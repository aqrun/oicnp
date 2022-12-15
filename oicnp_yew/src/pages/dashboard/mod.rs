use yew::prelude::*;
use crate::components::{
    InputTest, MainLayout, PageHeader,
    BreadItem, PageContent,
};

#[function_component]
pub fn Dashboard() -> Html {
    let bread_data: Vec<BreadItem> = vec![
        BreadItem { label: String::from("Home") },
        BreadItem { label: String::from("Dashboard") },
    ];

    html! {
        <MainLayout>
            <PageHeader
                bread_items={bread_data}
                title="Dashboard"
            />
            <PageContent>
                <InputTest />
                <hr />
                
                // <Button>{ "test" }</Button>
            </PageContent>
        </MainLayout>
    }
}

