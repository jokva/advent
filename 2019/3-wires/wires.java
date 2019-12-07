import java.io.*;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.ArrayList;
import java.util.List;

class Pos {
    Pos() {
        this.x = 0;
        this.y = 0;
    }

    Pos(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object obj) {
        Pos rhs = (Pos) obj;
        return this.x == rhs.x && this.y == rhs.y;
    }

    @Override
    public int hashCode() {
        return this.toString().hashCode();
    }

    public String toString() {
        return Integer.toString(this.x) + "," + Integer.toString(this.y);
    }


    public Pos left()  { return new Pos(this.x - 1, this.y);     }
    public Pos right() { return new Pos(this.x + 1, this.y);     }
    public Pos up()    { return new Pos(this.x,     this.y + 1); }
    public Pos down()  { return new Pos(this.x,     this.y - 1); }

    public int distance(Pos rhs) {
        return Math.abs(this.x - rhs.x) + Math.abs(this.y - rhs.y);
    }

    private int x;
    private int y;
}

class Cost implements Comparable< Cost > {
    private Integer cost;
    private Pos pos;

    Cost(Pos p, int c) {
        this.cost = c;
        this.pos = pos;
    }

    @Override
    public int compareTo(Cost c) {
        return this.cost.compareTo(c.cost);
    }

    public int latency() {
        return this.cost;
    }
}


public class wires {

    private static List< Pos > track_wire(String[] path) {
        List pos = new ArrayList< Pos >();
        Pos xy = new Pos();

        for (String p : path) {
            int steps = Integer.parseInt(p.substring(1));
            switch (p.charAt(0)) {
                case 'U':
                    for (int i = 0; i < steps; ++i) {
                        xy = xy.up();
                        pos.add(xy);
                    }
                    break;
                case 'D':
                    for (int i = 0; i < steps; ++i) {
                        xy = xy.down();
                        pos.add(xy);
                    }
                    break;
                case 'L':
                    for (int i = 0; i < steps; ++i) {
                        xy = xy.left();
                        pos.add(xy);
                    }
                    break;
                case 'R':
                    for (int i = 0; i < steps; ++i) {
                        xy = xy.right();
                        pos.add(xy);
                    }
                    break;
            }
        }

        return pos;
    }

    public static void main(String args[]) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        String[] fst = reader.readLine().split(",");
        String[] snd = reader.readLine().split(",");

        List< Pos > wire1 = track_wire(fst);
        List< Pos > wire2 = track_wire(snd);
        Set< Pos > cross = new HashSet< Pos >(wire1);
        cross.retainAll(new HashSet< Pos >(wire2));

        Pos origo = new Pos();
        int shortest = cross.stream()
            .map(x -> origo.distance(x))
            .min(Integer::compareTo)
            .get()
        ;

        int latency = cross.stream()
            .map(x -> new Cost(x, 2 + wire1.indexOf(x) + wire2.indexOf(x)))
            .min(Cost::compareTo)
            .map(Cost::latency)
            .get()
        ;

        System.out.println(shortest);
        System.out.println(latency);
    }
}
