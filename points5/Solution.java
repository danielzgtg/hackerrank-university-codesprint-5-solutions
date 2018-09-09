import java.util.*;

public class Solution {
	public static class Node {
		public int connections = 0;
		public List<Node> neighborhood = null;
	}
	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);
		String line;
		String[] split;
		line = scanner.nextLine();
		split = line.split(" ");
		int graphsize = Integer.parseInt(split[0]);
		int numedges = Integer.parseInt(split[1]);
		int a = Integer.parseInt(split[2]);
		int b = Integer.parseInt(split[3]);
		Node[] nodes = new Node[graphsize];
		List<Node>[] neighborhoods = new List[graphsize];
		for (int i = 0; i < graphsize; i++) {
			nodes[i] = new Node();
		}
		for (int i = 0; i < numedges; i++) {
			line = scanner.nextLine();
			split = line.split(" ");
			int idx1 = Integer.parseInt(split[0]) - 1;
			int idx2 = Integer.parseInt(split[1]) - 1;
			Node node1 = nodes[idx1];
			Node node2 = nodes[idx2];
			node1.connections++;
			node2.connections++;
			List<Node> neighborhood;
			if (node1.neighborhood == null) {
				if (node2.neighborhood == null) {
					neighborhood = new ArrayList<>();
					neighborhood.add(node1);
					neighborhood.add(node2);
					node1.neighborhood = neighborhood;
					node2.neighborhood = neighborhood;
					neighborhoods[idx1] = neighborhood;
				} else {
					neighborhood = node2.neighborhood;
					neighborhood.add(node1);
					node1.neighborhood = neighborhood;
				}
			} else {
				if (node2.neighborhood == null) {
					neighborhood = node1.neighborhood;
					neighborhood.add(node2);
					node2.neighborhood = neighborhood;
				} else {
					neighborhoods[idx2] = null;
					neighborhood = node1.neighborhood;
					List<Node> toMove = node2.neighborhood;
					for (Node node : toMove) {
						node.neighborhood = neighborhood;
					}
					neighborhood.addAll(toMove);
				}
			}
		}
		int count = 0;
		for (List<Node> neighborhood : neighborhoods) {
			if (neighborhood != null && !neighborhood.isEmpty()) {
				int min = 999999999;
				for (Node node : neighborhood) {
					int c = node.connections;
					if (c < min) {
						min = c;
					}
				}
				if (min != 0) {
					if (b > 1) {
						for (Node node : neighborhood) {
							if (node.connections > min) {
								count += 1;
							}
						}
					} else {
						int max = 0;
						for (Node node : neighborhood) {
							int c = node.connections;
							if (c > max) {
								max = c;
							}
						}
						for (Node node : neighborhood) {
							if (node.connections > min && node.connections < max) {
								count += 1;
							}
						}
					}
				}
			}
		}
		System.out.println(count);
	}
}
