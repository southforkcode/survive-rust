Feature: Rest Mechanic
  As a player
  I want to be able to rest
  So that I can regain health

  Scenario: Resting recovers health
    Given the game is running
    When I type the command "rest"
    Then the output should contain "You gained +20 health back."
    And the game should still be running
