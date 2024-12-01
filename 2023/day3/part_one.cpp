#include "part_one.h"

// ----------------------------------------------------------------------------------------------- Search ---
bool search_star(std::vector<std::vector<char>> &tab, uint i, uint j) {

  if (i < tab.size() - 2) {
    if (j > 0 && tab[i + 1][j - 1] == '*')
      return true;
    if (tab[i + 1][j] == '*')
      return true;
    if (j < tab[i].size() - 1 && tab[i + 1][j + 1] == '*')
      return true;
  }

  if (i > 0) {
    if (j > 0 && tab[i - 1][j - 1] == '*')
      return true;
    if (tab[i - 1][j] == '*')
      return true;
    if (j < tab[i].size() - 1 && tab[i - 1][j + 1] == '*')
      return true;
  }

  if (j > 0 && tab[i][j - 1] == '*')
    return true;
  if (j < tab[i].size() - 1 && tab[i][j + 1] == '*')
    return true;

  return false;
}

// ---------------------------------------------------------------------------------------------- Numbers ---
Numbers::Numbers() : _valid(false) {}

void Numbers::add(char c) {

  if (std::isdigit(c)) {
    _current += c;

  } else {
    if (!_current.empty() && _valid)
      _values.emplace_back(std::stoi(_current));

    _current.clear();
    _valid = false;
  }
}

bool Numbers::valid() const { return _valid; }
void Numbers::set_valid() { _valid = true; }
std::vector<int> &Numbers::values() { return _values; }

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
      for (auto &c : line) {
        if (!std::isdigit(c) && c != '.')
          c = '*';
        tab.back().push_back(c);
      }
    }
    file.close();
  }

  // --
  Numbers nums;

  for (int i = 0; i < tab.size(); ++i) {
    for (int j = 0; j < tab[i].size(); ++j) {

      if (std::isdigit(tab[i][j]) && !nums.valid() && search_star(tab, i, j))
        nums.set_valid();

      nums.add(tab[i][j]);
    }

    // Close the current line
    nums.add('.');
  }

  int total = 0;
  for (auto &n : nums.values())
    total += n;

  std::cout << total << "\n";

  return 0;
}
