#include <cctype>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

// ------------------------------------------------------------------------------------------ decorticate ---
int decorticate(std::vector<std::vector<char>> &tab, uint i, uint j) {

  // Go to the left up to the start or the next non digit
  std::string val;
  while (j > 0 && std::isdigit(tab[i][j - 1]))
    --j;

  while (j < tab[i].size() && std::isdigit(tab[i][j])) {
    val += tab[i][j];
    tab[i][j] = '.'; // Erase to help the second research
    ++j;
  }

  if (!val.empty())
    return std::stoi(val);

  return 0;
}

// ----------------------------------------------------------------------------------------------- Search ---
int search_number(std::vector<std::vector<char>> &tab, uint i, uint j) {

  if (i < tab.size() - 1) {
    if (j > 0 && std::isdigit(tab[i + 1][j - 1]))
      return decorticate(tab, i + 1, j - 1);
    if (std::isdigit(tab[i + 1][j]))
      return decorticate(tab, i + 1, j);
    if (j < tab[i].size() - 1 && std::isdigit(tab[i + 1][j + 1]))
      return decorticate(tab, i + 1, j + 1);
  }

  if (i > 0) {
    if (j > 0 && std::isdigit(tab[i - 1][j - 1]))
      return decorticate(tab, i - 1, j - 1);
    if (std::isdigit(tab[i - 1][j]))
      return decorticate(tab, i - 1, j);
    if (j < tab[i].size() - 1 && std::isdigit(tab[i - 1][j + 1]))
      return decorticate(tab, i - 1, j + 1);
  }

  if (j > 0 && std::isdigit(tab[i][j - 1]))
    return decorticate(tab, i, j - 1);
  if (j < tab[i].size() - 1 && std::isdigit(tab[i][j + 1]))
    return decorticate(tab, i, j + 1);

  return 0;
}

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::vector<char>> tab;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {
    while (getline(file, line)) {

      tab.push_back({});
      for (auto &c : line)
        tab.back().push_back(c);
    }
    file.close();
  }

  // --
  int total = 0;
  for (int i = 0; i < tab.size(); ++i) {
    for (int j = 0; j < tab[i].size(); ++j) {

      if (tab[i][j] == '*')
        total += search_number(tab, i, j) * search_number(tab, i, j);
    }
  }

  std::cout << total << "\n";

  return 0;
}
