#include <fstream>
#include <iostream>
#include <vector>

constexpr char EMPTY = '.';
constexpr char BORDER = '#';
constexpr char FILL = '0';

// ------------------------------------------------------------------------------------------------------
enum class Direction {
  NORTH,
  SOUTH,
  WEST,
  EAST,
};

struct Instruction {
  Direction direction;
  int length = 0;
  int color = 0;

  int x = 0;
  int y = 0;
};

// ------------------------------------------------------------------------------------------ Display ---
void display(std::vector<std::vector<char>> &plan) {

  for (const auto &ligne : plan) {
    for (const auto &c : ligne) {

      if (c == BORDER)
        std::cout << "\033[1;31m" << BORDER;
      else
        std::cout << "\033[1;30m" << c;

      std::cout << "\033[0m";
    }
    std::cout << "\n";
  }
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");
  // std::ifstream file("example.txt");
  // std::ifstream file("example2.txt");

  std::vector<Instruction> instructions;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      instructions.push_back({});

      if (!line.empty()) {
        for (const auto &c : line)

          // Direction
          if (line.front() == 'U')
            instructions.back().direction = Direction::NORTH;
          else if (line.front() == 'R')
            instructions.back().direction = Direction::EAST;
          else if (line.front() == 'D')
            instructions.back().direction = Direction::SOUTH;
          else
            instructions.back().direction = Direction::WEST;

        // Length
        std::string nb;
        for (int i = 1; i < 4; ++i) {
          if (std::isdigit(line[i]))
            nb += line[i];
        }
        instructions.back().length = std::stoi(nb);

        // Color
        nb.clear();
        for (int i = 4; i < line.size(); ++i) {
          if (std::isalnum(line[i]))
            nb += line[i];
        }
        instructions.back().color = std::stoi(nb, 0, 16);
      }
    }

    file.close();
  }

  // ------------------------------------------------------------ Convert instructions to coordinates ---
  Instruction previous; // 0
  for (auto it = instructions.begin(); it != instructions.end(); ++it) {

    if ((*it).direction == Direction::NORTH) {
      (*it).x = (previous).x - (*it).length;
      (*it).y = (previous).y;

    } else if ((*it).direction == Direction::EAST) {
      (*it).x = (previous).x;
      (*it).y = (previous).y + (*it).length;

    } else if ((*it).direction == Direction::SOUTH) {
      (*it).x = (previous).x + (*it).length;
      (*it).y = (previous).y;

    } else {
      (*it).x = (previous).x;
      (*it).y = (previous).y - (*it).length;
    }
    previous = *it;
  }

  // ----------------------------------------------------------------------- Find the smallet x and y ---
  int min_north = 0;
  int min_west = 0;
  for (auto &c : instructions) {

    if (c.x < min_west)
      min_west = c.x;
    if (c.y < min_north)
      min_north = c.y;
  }

  // ------------------------------------------------------------- Move all coordinates into the plan ---
  int width = 0;
  int heigh = 0;
  for (auto &c : instructions) {
    c.x += abs(min_west);
    c.y += abs(min_north);

    if (c.x > width)
      width = c.x;
    if (c.y > heigh)
      heigh = c.y;
  }

  // -------------------------------------------------------------------------------- Create the plan ---
  std::vector<std::vector<char>> plan(width + 1, std::vector<char>(heigh + 1, '.'));

  int row = abs(min_west);
  int column = abs(min_north);

  for (const auto &instruction : instructions) {
    for (int i = 0; i < instruction.length; ++i) {
      plan[row][column] = BORDER;

      if (instruction.direction == Direction::NORTH)
        row--;
      else if (instruction.direction == Direction::EAST)
        column++;
      else if (instruction.direction == Direction::SOUTH)
        row++;
      else
        column--;
    }
  }

  // ---------------------------------------------------------------------------------- Fill the plan ---
  // ---------------------------------------------------------------------------------------- & count ---
  int total = 0;
  for (int row = 0; row < plan.size(); ++row) {

    bool multiple_border = false;
    bool fill = false;

    bool angle_top = false;
    bool angle_bottom = false;

    for (int column = 0; column < plan[0].size(); ++column) {

      // Memorize the border angle
      if (plan[row][column] == BORDER) {
        angle_top = row > 0 && plan[row - 1][column] == BORDER;
        angle_bottom = row < plan.size() - 1 && plan[row + 1][column] == BORDER;
      }

      // Skip lignes of border
      while (plan[row][column] == BORDER && plan[row][column + 1] == BORDER) {
        multiple_border = true;
        ++column;
        total++;
      }

      // If both border angles are in the same side, reverse fill
      if (multiple_border) {
        if ((angle_top && plan[row - 1][column] == BORDER) ||
            (angle_bottom && plan[row + 1][column] == BORDER))
          fill = !fill;
      }

      if (plan[row][column] == BORDER) {
        fill = !fill;
        total++;
      }

      if (fill && plan[row][column] != BORDER) {
        plan[row][column] = FILL;
        total++;
      }

      // --
      multiple_border = false;
      angle_bottom = false;
      angle_top = false;
    }
  }

  // display(plan);
  std::cout << total << "\n";
  return 0;
}
