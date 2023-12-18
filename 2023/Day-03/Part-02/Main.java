import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		final int max_red = 12;
		final int max_green = 13;
		final int max_blue = 14;
		int sum = 0;
		Scanner input = new Scanner(System.in);
		List<String> input_lines = new ArrayList<>();
		while (input.hasNext()) {
			String line = input.nextLine();
			input_lines.add(line);
		}
		char[][] map = new char[input_lines.size()][];
        for (int i = 0; i < input_lines.size(); i++) {
            map[i] = input_lines.get(i).toCharArray();
        }
		int[][] number_ids = new int[map.length][map[0].length];
		int[][] numbers = new int[map.length][map[0].length];
		int number_id = 0;
		for(int r = 0; r < map.length; r += 1) {
			for(int c = 0; c < map[r].length; c += 1) {
				if(!Character.isDigit(map[r][c])) {
					number_ids[r][c] = -1;
					numbers[r][c] = 0;
					continue;
				}
				int cc = c;
				int number = 0;
				while(cc < map[r].length && Character.isDigit(map[r][cc])) {
					number *= 10;
					number += Character.getNumericValue(map[r][cc]);
					number_ids[r][cc] = number_id;
					cc += 1;
				}
				for(int i = c; i < cc; i += 1) {
					numbers[r][i] = number;
				}
				c = cc - 1;
				number_id += 1;
			}
		}
		for(int r = 0; r < map.length; r += 1) {
			for(int c = 0; c < map[r].length; c += 1) {
				if(map[r][c] != '*') {
					continue;
				}
				HashSet<Integer> gear_number_ids = new HashSet<>();
				int product = 1;
				for(int rt = Math.max(r - 1, 0); rt < Math.min(r + 1, map.length - 1) + 1; rt += 1) {
					for(int ct = Math.max(c - 1, 0); ct < Math.min(c + 1, map[rt].length - 1) + 1; ct += 1) {
						if(number_ids[rt][ct] < 0 || gear_number_ids.contains(number_ids[rt][ct])) {
							continue;
						}
						gear_number_ids.add(number_ids[rt][ct]);
						product *= numbers[rt][ct];
					}
				}
				if(gear_number_ids.size() == 2) {
					sum += product;
				}
			}
		}
		input.close();
		System.out.println(sum);
	}
}