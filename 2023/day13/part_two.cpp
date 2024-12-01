#include <fstream>
#include <iostream>
#include <vector>

// ----------------------------------------------------------------------------------------- Vertical ---
int match_two_columns(const std::vector<std::string> &pattern, int left, int right) {
  int smudge = 0;

  for (int row = 0; row < pattern.size(); ++row) {
    if (pattern[row][left] != pattern[row][right]) {
      ++smudge;
      if (smudge > 1)
        break;
    }
  }
  return smudge;
}

int64_t vertical(const std::vector<std::string> &pattern) {

  for (int column = 0; column < pattern[0].size() - 1; ++column) {

    int smudge = 0;
    for (int left = column, right = column + 1; left >= 0 && right < pattern[0].size(); --left, ++right) {
      smudge += match_two_columns(pattern, left, right);

      if (smudge > 1)
        break;
    }

    if (smudge == 1)
      return column + 1;
  }
  return 0;
}

// --------------------------------------------------------------------------------------- Horizontal ---
int match_two_row(const std::vector<std::string> &pattern, int top, int bottom) {
  int smudge = 0;

  for (int column = 0; column < pattern[0].size(); ++column) {
    if (pattern[top][column] != pattern[bottom][column]) {
      ++smudge;
      if (smudge > 1)
        break;
    }
  }
  return smudge;
}

int64_t horizontal(const std::vector<std::string> &pattern) {

  for (int row = 0; row < pattern.size() - 1; ++row) {

    int smudge = 0;
    for (int top = row, bottom = row + 1; top >= 0 && bottom < pattern.size(); --top, ++bottom) {
      smudge += match_two_row(pattern, top, bottom);

      if (smudge > 1)
        break;
    }

    if (smudge == 1)
      return row + 1;
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

  int64_t total = 0;
  for (const auto &p : patterns) {
    total += horizontal(p) * 100;
    total += vertical(p);
  }

  std::cout << "total : " << total << "\n";
  return 0;
}
