import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

// sample program to mimic a memory leak
public class Filler {
    static List<UUID> someList = new ArrayList<>();

    public static void main(String[] args) {
        while (true) {
            someList.add(UUID.randomUUID());
        }
    }
}
