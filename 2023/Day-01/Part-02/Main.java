import 	java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		String[] word_numbers = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
		int sum = 0;
		Scanner input = new Scanner(System.in);
		while(input.hasNext()) {
			String line = input.nextLine();
			int first_index = line.length();
			int last_index_from_end = line.length();
			int first_digit = 0;
			int last_digit = 0;
			for(int i = 0; i < word_numbers.length; i += 1) {
				String num_word = word_numbers[i];
				String num_str = Integer.toString(i);
				String line_temp = line.replaceAll(num_word, num_str);
				int first_i = line_temp.indexOf(num_str);
				if(first_i >= 0 && first_i < first_index) {
					first_index = first_i;
					first_digit = i;
				}
				int last_i = new StringBuffer(line_temp).reverse().toString().indexOf(num_str);
				if(last_i >= 0 && last_i < last_index_from_end) {
					last_index_from_end = last_i;
					last_digit = i;
				}
			}	
			sum += first_digit * 10 + last_digit;
		}
		input.close();
		System.out.println(sum);
	}
}