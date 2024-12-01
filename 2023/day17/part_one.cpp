#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

constexpr char EMPTY = '.';
constexpr char MIRROR_T1 = '/';
constexpr char MIRROR_T2 = '\\';
constexpr char FLAT = '-';
constexpr char SPLITTER = '|';

constexpr char DONE = 'X';
// ------------------------------------------------------------------------------------------------------
enum class Direction {
  NORTH = 1,
  SOUTH = 2,
  WEST = 4,
  EAST = 8,
};

// struct Block {
//   int value;
//   char visited = '0';
// };

struct History {
  int row = -1;
  int column = -1;
  int count = 0;
};

struct Counter {
  Direction direction = Direction::NORTH;
  uint16_t count = 0;
};

struct Possibility {

  int value;
  int row;
  int column;
};

// ------------------------------------------------------------------------------------------ Display ---
void display(std::vector<std::vector<char>> &map) {

  for (const auto &ligne : map) {
    for (const auto &c : ligne) {

      // std::cout << c;
      if (c == DONE) {
        std::cout << "\033[1;31m" << DONE;
      } else {
        std::cout << "\033[1;30m" << c;
      }
      std::cout << "\033[0m";
    }
    std::cout << "\n";
  }
}

bool is_smallest(std::vector<std::vector<char>> &map, int row, int column, int current_value, int &smallest) {

  if (current_value >= smallest) {
    // std::cout << "Close this one because value is already too high\n";
    return false;
  }

  // int estimation = current_value + (map.size() - row) * 1 + (map[0].size() - column) * 1;
  int estimation = current_value + (map.size() - row) * 2 + (map[0].size() - column) * 2 - 2;
  // std::cout << "Estimation: " << estimation << " - Smallest: " << smallest << "\n";
  if (estimation > smallest) {
    // std::cout << "Estimation: " << estimation << " - Smallest: " << smallest << "\n";
    // display(map);
    return false;
  }

  if (row == map.size() - 1 && column == map[0].size() - 1) {
    // std::cout << "Last case reached with the value ------------------------------------" << current_value
    // << "\n";
    display(map);
    if (current_value < smallest) {
      display(map);
      smallest = current_value;
      std::cout << "Replace the smallest !! " << smallest << " \n";
    }
    return true;
  }

  // display(map);
  // std::this_thread::sleep_for(std::chrono::milliseconds(1000));

  return true;
}

// --------------------------------------------------------------------------------------------- Blow ---
void run(std::vector<std::vector<char>> map, int row, int column, int current_value, int &smallest,
         Counter counter) {

  // std::cout << "Valeur de " << row << "/" << column << "    " << map[row][column] << " <--\n";

  // display(map);
  // std::this_thread::sleep_for(std::chrono::milliseconds(1000));

  // if (row != 0 || column != 0) {
  current_value += map[row][column] - '0';
  map[row][column] = DONE;
  // }

  if (!is_smallest(map, row, column, current_value, smallest)) {
    return;
  }

  if (column < map[0].size() - 1 && map[row][column + 1] != DONE) { // East
    Counter new_counter = counter;
    if (new_counter.direction == Direction::EAST) {
      new_counter.count++;
    } else {
      new_counter.direction = Direction::EAST;
      new_counter.count = 0;
    }

    if (new_counter.count < 3)
      run(map, row, column + 1, current_value, smallest, new_counter);
  }

  if (row < map.size() - 1 && map[row + 1][column] != DONE) { // South
    Counter new_counter = counter;
    if (new_counter.direction == Direction::SOUTH) {
      new_counter.count++;
    } else {
      new_counter.direction = Direction::SOUTH;
      new_counter.count = 0;
    }

    if (new_counter.count < 3)
      run(map, row + 1, column, current_value, smallest, new_counter);
  }

  if (row > 0 && map[row - 1][column] != DONE) { // North
    Counter new_counter = counter;
    if (new_counter.direction == Direction::NORTH) {
      new_counter.count++;
    } else {
      new_counter.direction = Direction::NORTH;
      new_counter.count = 0;
    }

    if (new_counter.count < 3)
      run(map, row - 1, column, current_value, smallest, new_counter);
  }

  if (column > 0 && map[row][column - 1] != DONE) { // West
    Counter new_counter = counter;
    if (new_counter.direction == Direction::WEST) {
      new_counter.count++;
    } else {
      new_counter.direction = Direction::WEST;
      new_counter.count = 0;
    }

    if (new_counter.count < 3)
      run(map, row, column - 1, current_value, smallest, new_counter);
  }

  // std::cout << "Close this one\n";
  // return;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");
  // std::ifstream file("example.txt");

  std::vector<std::vector<char>> map;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        map.push_back({});
        for (const auto &c : line)
          // map.back().push_back({.value = c - '0'});
          map.back().push_back(c);
      }
    }

    file.close();
  }

  display(map);
  // ---------------------------------------------------------------------------------------- Process ---

  int smallest = 9 * map.size() + 9 * map[0].size();
  // int smallest = 107;
  std::cout << smallest << "\n";
  // return 0;

  Counter counter;
  run(map, 1, 0, 0, smallest, {.direction = Direction::SOUTH, .count = 1});
  run(map, 0, 1, 0, smallest, {.direction = Direction::EAST, .count = 1});
  // run(map, 0, 1, 0, smallest, counter);

  // int64_t total = 0;
  std::cout << smallest << "\n";
  return 0;
}
