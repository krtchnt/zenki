#![allow(clippy::too_many_lines)]
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    route::{ITEM, USER},
    transaction::{TransactionError, TransactionParams, get_transaction},
};

#[component]
pub fn Transaction() -> impl IntoView {
    let query = use_params::<TransactionParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.unwrap_or_default())
                .map_err(|_| TransactionError::InvalidId)
        })
    };
    let transaction_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(tname) => get_transaction(tname)
                .await
                .map(|data| data.ok_or(TransactionError::TransactionNotFound))
                .map_err(|_| TransactionError::ServerError),
        }
    });
    let transaction_view = Suspend::new(async move {
        match transaction_resource.await {
            Ok(Ok(tx)) => {
                Ok(view! {
                    <h2>{tx.bought_at.clone().unwrap_or_else(|| String::from("<no bought timestamp provided>"))}</h2>
                    <p><b>Amount: </b>{tx.amount}</p>
                    <p><b>Sender: </b><a href={format!("{}/{}", USER, tx.uid)}>{tx.s_uname}</a></p>
                    <p><b>Receiver: </b><a href={format!("{}/{}", USER, tx.receiver_uid.unwrap_or_default())}>{tx.r_uname}</a></p>
                    <p><b>Item: </b><a href={format!("{}/{}", ITEM, tx.pid)}>{tx.p_descr}</a></p>
                    <p><b>Payment Method: </b>{tx.payment_method}</p>
                    <p><b>Status: </b>{tx.status.clone().unwrap_or_else(|| String::from("<no statys provided>"))}</p>

                    /*
                    pub uid: usize,
                    pub receiver_uid: Option<usize>,
                    pub pid: usize,
                    */

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=tx.tid.to_string()/>
                    <Meta name="description" content=tx.bought_at.unwrap_or_else(|| String::from("<no bought timestamp provided>"))/>
                })
            }
            _ => Err(TransactionError::ServerError),
        }
    });

    view! {
        <h1>"Transaction Info"</h1>
        <Suspense fallback=move || view! { <p>"Loading transaction..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>{transaction_view}</ErrorBoundary>
        </Suspense>
    }
}
