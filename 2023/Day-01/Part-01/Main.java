import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		int sum = 0;
		Scanner input = new Scanner(System.in);
		while (input.hasNext()) {
			String line = input.nextLine();
			int[] numbers = line.chars().filter(Character::isDigit).map(Character::getNumericValue).toArray();
			sum += numbers[0] * 10 + numbers[numbers.length - 1];
		}
		input.close();
		System.out.println(sum);
	}
}