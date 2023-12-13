#include <fstream>
#include <iostream>
#include <vector>

// ------------------------------------------------------------------------------------------ Display ---
void display_patterns(const std::vector<std::vector<std::string>> &patterns) {

  for (const auto &line : patterns) {
    for (const auto &c : line) {
      std::cout << c << "\n";
    }
  }
}

// ----------------------------------------------------------------------------------------- Vertical ---
bool match_two_columns(const std::vector<std::string> &pattern, int left, int right) {
  for (int row = 0; row < pattern.size(); ++row) {
    if (pattern[row][left] != pattern[row][right])
      return false;
  }
  return true;
}

int64_t vertical(const std::vector<std::string> &pattern) {

  for (int column = 0; column < pattern[0].size() - 1; ++column) {
    if (match_two_columns(pattern, column, column + 1)) {

      bool ok = true;
      for (int to_l = column - 1, to_r = column + 2; to_l >= 0 && to_r < pattern[0].size(); --to_l, ++to_r) {
        if (!match_two_columns(pattern, to_l, to_r)) {
          ok = false;
          break;
        }
      }
      if (ok)
        return column + 1;
    }
  }
  return 0;
}

// --------------------------------------------------------------------------------------- Horizontal ---
int64_t horizontal(const std::vector<std::string> &pattern) {

  for (int row = 0; row < pattern.size() - 1; ++row) {
    if (pattern[row] == pattern[row + 1]) {

      bool ok = true;
      for (int to_t = row - 1, to_b = row + 2; to_t >= 0 && to_b < pattern.size(); --to_t, ++to_b) {
        if (pattern[to_t] != pattern[to_b]) {
          ok = false;
          break;
        }
      }
      if (ok)
        return row + 1;
    }
  }
  return 0;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::vector<std::vector<std::string>> patterns;

  std::string line;
  std::ifstream file("input.txt");

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    patterns.push_back({});
    while (getline(file, line)) {
      if (!line.empty())
        patterns.back().emplace_back(std::move(line));
      else
        patterns.push_back({});
    }

    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  // display_patterns(patterns);

  int64_t total = 0;
  for (const auto &p : patterns) {
    total += vertical(p);
    total += horizontal(p) * 100;
  }

  std::cout << "total : " << total << "\n";
  return 0;
}
