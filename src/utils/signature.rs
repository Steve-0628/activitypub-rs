use reqwest::Request;


pub(crate) async fn sign_to_reqest(req:&mut Request) -> &mut Request {
	let mut headers = req.headers_mut();

	req
}