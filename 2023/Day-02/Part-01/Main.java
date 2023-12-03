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
			boolean possible = true;
			for(String pull : pulls) {
				pull = pull.trim();
				for(String single_pull : pull.split(",")) {
					single_pull = single_pull.trim();
					String[] split_single_pull = single_pull.split(" ");
					int amount = Integer.parseInt(split_single_pull[0]);
					String colour = split_single_pull[1].trim();
					if(colour.equalsIgnoreCase("red")) {
						possible = (amount <= max_red);
					} else if(colour.equalsIgnoreCase("green")) {
						possible = (amount <= max_green);
					} else if(colour.equalsIgnoreCase("blue")) {
						possible = (amount <= max_blue);
					}
					if(!possible) {
						break;
					}
				}
				if(!possible) {
					break;
				}
			}
			if(possible) {
				sum += game_id;
			}
		}
		input.close();
		System.out.println(sum);
	}
}