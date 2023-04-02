use crate::view;
use crate::BoltApp;
use crate::Msg;
use crate::GLOBAL_STATE;
use yew::{html, Context, Html};

pub fn request(ctx: &Context<BoltApp>) -> Html {
    let state = GLOBAL_STATE.lock().unwrap();

    html! {
        <div class="req">
            <div class="requestbar">
                <div class="">
                    <select id="methodselect" class="methodselect pointer" onchange={ctx.link().callback(|_| Msg::MethodChanged)}>
                        <option value="get">{"GET"}</option>
                        <option value="post">{"POST"}</option>
                    </select>
                </div>

                <div>
                    <input id="urlinput" class="urlinput" type="text" value={state.request.url.clone()} placeholder="http://" onchange={ctx.link().callback(|_| Msg::UrlChanged)}/>
                </div>

                <button class="sendbtn pointer" type="button" onclick={ctx.link().callback(|_| Msg::SendPressed)}>{"Send"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class=" tab pointer tabSelected" onclick={ctx.link().callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
                <div id="req_params_tab" class=" tab pointer" onclick={ctx.link().callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class=" tab pointer" onclick={ctx.link().callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if state.req_tab == 1 {
                    <textarea id="reqbody" class="reqbody" value={state.request.body.clone()} placeholder="Request body" onchange={ctx.link().callback(|_| Msg::BodyChanged)}>

                    </textarea>
                } else if state.req_tab == 2 {
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for state.request.params.iter().enumerate().map(|(index, header)| view::param::render_params(ctx, index, state.request.params.len(), &header[0], &header[1])) }
                    </table>
                } else if state.req_tab == 3 {
                    <table>
                        <tr>
                            <th>{"Header"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for state.request.headers.iter().enumerate().map(|(index, header)| view::header::render_reqheader(ctx, index, state.request.headers.len(), &header[0], &header[1])) }
                    </table>
                }
            </div>
        </div>

    }
}