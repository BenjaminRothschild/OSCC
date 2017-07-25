# language: en

Feature: Sending reports

  Brake reports should be published to the control CAN bus after an interval.


  Scenario: Brake report published after an interval
    When a brake report is published

    Then a brake report should be put on the control CAN bus
    And the brake report's enabled field should be set
    And the brake report's override field should be set
    And the brake report's DTCs field should be set
    And the brake report's front pressure sensor fields should be set
