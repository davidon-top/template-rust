{%- if async -%}
#[tokio::main]
async fn main() {
	println!("Hello, world!");
}
{%- else -%}
fn main() {
	println!("Hello, world!");
}
{%- endif -%}
