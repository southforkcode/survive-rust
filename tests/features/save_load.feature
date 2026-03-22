Feature: Save/Load Game State
  Scenario: Save and load the game
    Given the game is running
    When I type the command "gather wood"
    Then the output should contain "Gathered"
    When I type the command "save tests/data/test_save_bdd.yaml"
    Then the output should contain "Game saved to tests/data/test_save_bdd.yaml"
    When I type the command "load tests/data/test_save_bdd.yaml"
    Then the output should contain "Game state restored from tests/data/test_save_bdd.yaml"
    When I type the command "status camp"
    Then the output should contain "wood"
