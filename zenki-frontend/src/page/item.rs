#![allow(clippy::too_many_lines)]
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    item::{ItemError, ItemParams, get_item},
    transaction::create_transaction,
    user::{UserError, get_users},
};

#[component]
pub fn Item() -> impl IntoView {
    let query = use_params::<ItemParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.unwrap_or_default())
                .map_err(|_| ItemError::InvalidId)
        })
    };
    let item_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(tname) => get_item(tname)
                .await
                .map(|data| data.ok_or(ItemError::ItemNotFound))
                .map_err(|_| ItemError::ServerError),
        }
    });
    let item_view = Suspend::new(async move {
        match item_resource.await {
            Ok(Ok(item)) => Ok(view! {
                <h2>{item.descr.clone()}</h2>
                <p><b>Description: </b>{item.descr.clone().unwrap_or_else(|| String::from("<no description provided>"))}</p>
                <p><b>Price: </b>{item.price}{" USD"}</p>
                <p><b>Type: </b>{item.kind}</p>
                <p><b>Created At: </b>{item.created_at.clone().unwrap_or_else(|| String::from("<no creation time provided>"))}</p>

                <Title text=item.pid.to_string()/>
                <Meta name="description" content=item.descr.unwrap_or_default()/>
            }),
            _ => Err(ItemError::ServerError),
        }
    });

    let cookie = crate::auth::get_login_session().0;
    let uid = move || cookie.get();
    let receiver_uid = RwSignal::new(uid().unwrap_or_default().to_string());
    let amount = RwSignal::new(String::from("1"));
    let payment_method = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if let (Some(uid), Ok(pid), Ok(ruid), Ok(amount_usize)) = (
                uid(),
                id(),
                receiver_uid.get().parse(),
                amount.get().parse(),
            ) {
                if create_transaction(uid, pid, ruid, payment_method.get(), amount_usize)
                    .await
                    .is_ok()
                {
                    receiver_uid.set(String::new());
                    amount.set(String::new());
                    payment_method.set(String::new());
                }
            }
        });
    };

    let users_resource = Resource::new(
        || (),
        |()| async move { get_users().await.map_err(|_| UserError::ServerError) },
    );
    let checkout_view = Suspend::new(async move {
        (users_resource.await).map_or(Err(UserError::ServerError), |users| {
            Ok(view! {
                <h3>"Checkout"</h3>
                <form on:submit=on_submit>
                    <div>
                        <label for="receiver_id">"Receiver UID:"</label>
                        <select id="receiver_id" bind:value=receiver_uid>{
                            users
                                .into_iter()
                                .map(|user| view! {
                                    <option value={user.uid}>{user.uname}</option>
                                })
                                .collect_view().into_any()
                        }</select>
                    </div>
                    <div>
                        <label for="amount">"Amount:"</label>
                        <input
                            id="amount"
                            type="number"
                            min="1"
                            bind:value=amount
                        />
                    </div>
                    <div>
                        <label for="amount">"Payment Method:"</label>
                        <select id="payment_method" bind:value=payment_method>
                            <option value="credit_card">Credit Card</option>
                            <option value="debit_card">Debit Card</option>
                            <option value="paypal">PayPal</option>
                            <option value="etc">etc.</option>
                        </select>
                    </div>
                    <div>
                        <button type="submit">"Pay"</button>
                    </div>
                </form>
            })
        })
    });

    view! {
        <h1>"Item Info"</h1>
        <Suspense fallback=move || view! { <p>"Loading item..."</p> }>
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
            }>{item_view}</ErrorBoundary>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading checkout..."</p> }>{checkout_view}</Suspense>
    }
}
