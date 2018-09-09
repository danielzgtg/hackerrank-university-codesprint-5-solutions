
import java.util.*;

public class Solution {
	public static class Node {
		public List<Node> edges = new ArrayList<>();
		public List<Node> revedges = new ArrayList<>();
		public int dist = 999999;
		public final char name;
		public Node(char name) {
			this.name = name;
		}
		public void connect(Node other) {
			if (!this.edges.contains(other)) {
				this.edges.add(other);
				other.revedges.add(this);
			}
		}
		public void calcDistancesTo() {
			this.dist = 0;
			List<Node> heatmap1 = new LinkedList<>();
			List<Node> heatmap2 = new LinkedList<>();
			heatmap1.add(this);
			while (!heatmap1.isEmpty()) {
				for (Node me : heatmap1) {
					int newdist = me.dist + 1;
					for (Node other : me.revedges) {
						if (other.dist > newdist) {
							other.dist = newdist;
							if (!heatmap2.contains(other)) {
								heatmap2.add(other);
							}
						}
					}
				}
				List<Node> heatmap4 = heatmap1;
				heatmap4.clear();
				heatmap1 = heatmap2;
				heatmap2 = heatmap4;
			}
		}
		public String recurse(String prev) {
			String cur = prev + this.name;
			if (this.dist == 0) {
				return cur;
			}
			Iterator<Node> iter = this.edges.iterator();
			String result = iter.next().recurse(cur);
			while (iter.hasNext()) {
				Node next = iter.next();
				String newresult = next.recurse(cur);
				if (newresult.compareTo(result) < 0) {
					result = newresult;
				}
			}
			return result;
		}
	}
	public static void prune(Node[] nodes) {
		for (Node me : nodes) {
			if (me.dist == 0) continue;
			int expect = me.dist - 1;
			me.edges.removeIf((x) -> x.dist != expect);
			if (!me.edges.isEmpty()) {
				int min = me.edges.stream().mapToInt((x) -> x.name).min().getAsInt();
				me.edges.removeIf((x) -> x.name != min);
			}
		}
	}
	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);
		String line;
		String[] split;
		line = scanner.nextLine();
		split = line.split(" ");
		int graphsize = Integer.parseInt(split[0]);
		int numedges = Integer.parseInt(split[1]);
		line = scanner.nextLine();
		Node[] nodes = new Node[graphsize];
		char[] names = line.toCharArray();
		for(int i = 0; i < graphsize; i++) {
			nodes[i] = new Node(names[i]);
		}
		for (int i = 0; i < numedges; i++) {
			line = scanner.nextLine();
			split = line.split(" ");
			int idx1 = Integer.parseInt(split[0]) - 1;
			int idx2 = Integer.parseInt(split[1]) - 1;
			nodes[idx1].connect(nodes[idx2]);
		}
		line = scanner.nextLine();
		split = line.split(" ");
		Node s = nodes[Integer.parseInt(split[0]) - 1];
		Node f = nodes[Integer.parseInt(split[1]) - 1];
		f.calcDistancesTo();
		prune(nodes);
		if (s.dist != 999999) {
			System.out.println(s.recurse(""));
		} else {
			System.out.println("No way");
		}
	}
}


