#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

constexpr char EMPTY = '.';
constexpr char MIRROR_T1 = '/';
constexpr char MIRROR_T2 = '\\';
constexpr char FLAT = '-';
constexpr char SPLITTER = '|';

// ------------------------------------------------------------------------------------------------------
enum class Direction {
  UP = 1,
  DOWN = 2,
  LEFT = 4,
  RIGHT = 8,
};

struct Tile {
  char value = '.';
  int energized = 0; // Mask with Direction
};

// ------------------------------------------------------------------------------------------ Display ---
void display(std::vector<std::vector<Tile>> &contraption) {

  for (const auto &ligne : contraption) {
    for (const auto &c : ligne) {

      if (c.energized > 0) {
        std::cout << "\033[1;31m#";

      } else {
        switch (c.value) {
        case EMPTY:
          std::cout << "\033[1;30m";
          break;
        case MIRROR_T1:
        case MIRROR_T2:
          std::cout << "\033[1;35m";
          break;
        case FLAT:
        case SPLITTER:
          std::cout << "\033[1;36m";
          break;
        default:
          std::cout << "\033[1;33m";
          break;
        }
        std::cout << c.value << "\033[0m";
      }
    }
    std::cout << "\n";
  }
}

// --------------------------------------------------------------------------------------------- Blow ---
void blow(std::vector<std::vector<Tile>> &contraption, int row, int column, Direction direction) {

  if (column != -1) { // Avoid start

    // Check & set the energized bit according to direction to avoid infinite loops
    if (contraption[row][column].energized & (int)direction)
      return;
    else
      contraption[row][column].energized += (int)direction;
  }

  // Move and check if where are on a border
  if (direction == Direction::UP)
    --row;
  else if (direction == Direction::DOWN)
    ++row;
  else if (direction == Direction::LEFT)
    --column;
  else
    ++column;

  if (row < 0 || row > contraption.size() - 1 || column < 0 || column > contraption.size() - 1)
    return;

  // Launch the next step
  switch (contraption[row][column].value) {

  case MIRROR_T1:
    if (direction == Direction::UP)
      return blow(contraption, row, column, Direction::RIGHT);
    else if (direction == Direction::DOWN)
      return blow(contraption, row, column, Direction::LEFT);
    else if (direction == Direction::LEFT)
      return blow(contraption, row, column, Direction::DOWN);
    else
      return blow(contraption, row, column, Direction::UP);

  case MIRROR_T2:
    if (direction == Direction::UP)
      return blow(contraption, row, column, Direction::LEFT);
    else if (direction == Direction::DOWN)
      return blow(contraption, row, column, Direction::RIGHT);
    else if (direction == Direction::LEFT)
      return blow(contraption, row, column, Direction::UP);
    else
      return blow(contraption, row, column, Direction::DOWN);

  case FLAT:
    if (direction == Direction::UP || direction == Direction::DOWN) {
      blow(contraption, row, column, Direction::LEFT);
      blow(contraption, row, column, Direction::RIGHT);
      return;
    } else
      return blow(contraption, row, column, direction);

  case SPLITTER:
    if (direction == Direction::LEFT || direction == Direction::RIGHT) {
      blow(contraption, row, column, Direction::UP);
      blow(contraption, row, column, Direction::DOWN);
      return;
    } else
      return blow(contraption, row, column, direction);

  default:
    return blow(contraption, row, column, direction);
  }
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");
  // std::ifstream file("example.txt");

  std::vector<std::vector<Tile>> contraption;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        contraption.push_back({});
        for (const auto &c : line)
          contraption.back().push_back({.value = c});
      }
    }

    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  // display(contraption);
  blow(contraption, 0, -1, Direction::RIGHT);

  display(contraption);

  int64_t total = 0;
  for (const auto &l : contraption) {
    for (const auto &c : l) {
      if (c.energized > 0)
        ++total;
    }
  }

  std::cout << total << "\n";
  return 0;
}
