pub mod node;


#[cfg(test)]
mod tests	{
	
	use super::node;

	#[test]
	fn it_works()	{
		let n = node::create_node(3);
		println!("Test is Done ____________");
		//println!("{:#?}", n);
	}
}
