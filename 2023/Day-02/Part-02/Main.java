import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		final int max_red = 12;
		final int max_green = 13;
		final int max_blue = 14;
		int sum = 0;
		Scanner input = new Scanner(System.in);
		while (input.hasNext()) {
			String line = input.nextLine();
			String[] game_pulls = line.split(":");
			int game_id = Integer.parseInt(game_pulls[0].split(" ")[1]);
			String[] pulls = game_pulls[1].split(";");
			int[] minimum_amounts = {0, 0, 0}; // [red, green, blue]
			for(String pull : pulls) {
				pull = pull.trim();
				for(String single_pull : pull.split(",")) {
					single_pull = single_pull.trim();
					String[] split_single_pull = single_pull.split(" ");
					int amount = Integer.parseInt(split_single_pull[0]);
					String colour = split_single_pull[1].trim();
					if(colour.equalsIgnoreCase("red")) {
						if(amount > minimum_amounts[0]) {
							minimum_amounts[0] = amount;
						}
					} else if(colour.equalsIgnoreCase("green")) {
						if(amount > minimum_amounts[1]) {
							minimum_amounts[1] = amount;
						}
					} else if(colour.equalsIgnoreCase("blue")) {
						if(amount > minimum_amounts[2]) {
							minimum_amounts[2] = amount;
						}
					}
				}
			}
			sum += minimum_amounts[0] * minimum_amounts[1] * minimum_amounts[2];
		}
		input.close();
		System.out.println(sum);
	}
}