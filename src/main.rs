const MAX: usize = 5000;

fn main() {

    //First generate the first 25 ludic numbers
    let mut result: Vec<i32> = vec![];

    result.push(1);
    let mut array: [i32; MAX] = [0; MAX];

    //Construct the first array
    let mut num_inicial:i32 = 2;
    for i in 0..array.len()
    {
    	array[i] = num_inicial;
    	num_inicial += 1;
    }
    
    //Show first 25 Ludic numbers
    //Calculate n Ludic numbers

    for _ in 0..5000
    {
    	let next_ludic = array[0];
    	result.push(next_ludic);
    	//print!("{},", next_ludic);
    	let mut contador = 0;    	
	    for i in 0..array.len()
	    {
	    	//modulo operation:
	    	//((a % b) + b) % b
	    	let modulo = ((i as i32 % next_ludic) + next_ludic) % next_ludic;
	    	//print!("{},",modulo);
	    	if modulo != 0
	    	{	    		
	    		array[contador] = array[i];	    		
	    		contador += 1;
	    	}
	    }	    
	    //
	}
	//println!("");
	print_n_ludics(&result, 25);
	print_num_ludics_upto(&result, 1000);
	print_ludics_from_to(&result, 2000, 2005)
	
}

fn print_array(x: &[i32; MAX])
{
	for i in 0..x.len()
	{
		print!("{},", x[i]);
	}
	println!("");
}

fn print_n_ludics(x: &Vec<i32>, n: i32)
{
	for i in 0..25
	{
		print!("{},", x[i]);
	}
	println!("");
}

fn print_num_ludics_upto(x: &Vec<i32>, max_num: i32)
{
	let mut num: i32 = 0;
	for i in 0..x.len()
	{
		if x[i] < max_num
		{
			//print!("{},", x[i]);
			num = num + 1;
		}
	}
	println!("{}", num);
}

fn print_ludics_from_to(x: &Vec<i32>, from: i32, to: i32)
{
	let mut num: i32 = 0;
	for i in 2000..2005
	{		
		print!("{},", x[i]);			
	}
	println!("");
}
