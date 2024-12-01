#include <fstream>
#include <iostream>
#include <string>
#include <vector>

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::vector<int>> tab;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {
    while (getline(file, line)) {

      // ------------------------------------------------------------------------------- Build the array ---
      // ex:        'Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1'
      // To obtain: 'nb matching numbers':1

      std::string nb;
      int field = 0;

      std::vector<std::string> field_1;
      std::string field_2;

      for (const auto &c : line) {

        if (field == 1) {
          if (nb.length() == 3 && c == ' ') { // Match 3 chars, no int conversion
            field_1.push_back(nb);
            nb.clear();
          }

          nb += c;

        } else if (field == 2)
          field_2 += c;

        if (c == ':')
          field = 1;
        if (c == '|')
          field = 2;
      }

      int nb_matching = 0;
      for (auto &c : field_1) {
        if (field_2.find(c) != std::string::npos)
          ++nb_matching;
      }

      tab.push_back({nb_matching, 1});
    }
    file.close();
  }

  // ------------------------------------------------------------------------------------ Fill the array ---
  for (int i = 0; i < tab.size(); ++i) {

    for (int j = 0; j < tab[i][1]; ++j) {

      for (int k = 1; k <= tab[i][0]; ++k) {

        tab[i + k][1]++;
      }
    }
  }

  // ------------------------------------------------------------------------------------- Add all cards ---
  int total = 0;
  for (const auto &c : tab)
    total += c[1];

  std::cout << total << "\n";

  return 0;
}
