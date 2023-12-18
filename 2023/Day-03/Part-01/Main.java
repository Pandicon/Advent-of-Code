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
		HashSet<String> added_positions = new HashSet<>();
		for(int r = 0; r < map.length; r += 1) {
			for(int c = 0; c < map[r].length; c += 1) {
				if(map[r][c] == '.' || Character.isDigit(map[r][c])) {
					continue;
				}
				for(int rt = Math.max(r - 1, 0); rt < Math.min(r + 1, map.length - 1) + 1; rt += 1) {
					for(int ct = Math.max(c - 1, 0); ct < Math.min(c + 1, map[rt].length - 1) + 1; ct += 1) {
						if(!Character.isDigit(map[rt][ct])) {
							continue;
						}
						String id = rt + "-" + ct;
						if(added_positions.contains(id)) {
							continue;
						}
						added_positions.add(id);
						List<Integer> beginning = new ArrayList<>();
						int middle = Character.getNumericValue(map[rt][ct]);
						List<Integer> end = new ArrayList<>();
						int ci = ct - 1;
						while(ci >= 0 && Character.isDigit(map[rt][ci])) {
							added_positions.add(rt + "-" + ci);
							beginning.add(Character.getNumericValue(map[rt][ci]));
							ci -= 1;
						}
						ci = ct + 1;
						while(ci < map[rt].length && Character.isDigit(map[rt][ci])) {
							added_positions.add(rt + "-" + ci);
							end.add(Character.getNumericValue(map[rt][ci]));
							ci += 1;
						}
						int number = 0;
						for(int i = beginning.size() - 1; i >= 0; i -= 1) {
							number *= 10;
							number += beginning.get(i);
						}
						number *= 10;
						number += middle;
						for(int i = 0; i < end.size(); i += 1) {
							number *= 10;
							number += end.get(i);
						}
						sum += number;
					}
				}
			}
		}
		input.close();
		System.out.println(sum);
	}
}