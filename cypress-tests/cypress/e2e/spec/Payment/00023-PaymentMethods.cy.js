import * as fixtures from "../../../fixtures/imports";
import State from "../../../utils/State";
import getConnectorDetails, * as utils from "../../configs/Payment/Utils";

let globalState;
let saveCardBody;

describe("Payment Methods Tests", () => {
  before("seed global state", () => {
    cy.task("getGlobalState").then((state) => {
      globalState = new State(state);
    });
  });

  after("flush global state", () => {
    cy.task("setGlobalState", globalState.data);
  });

  context("Create payment method for customer", () => {
    it("Create customer", () => {
      cy.createCustomerCallTest(fixtures.customerCreateBody, globalState);
    });

    it("Create Payment Method", () => {
      const data = getConnectorDetails("commons")["card_pm"]["PaymentMethod"];

      cy.createPaymentMethodTest(globalState, data);
    });

    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });
  });

  context("Set default payment method", () => {
    let shouldContinue = true;

    beforeEach(function () {
      if (!shouldContinue) {
        this.skip();
      }
    });

    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    it("Create Payment Method", () => {
      const data = getConnectorDetails("commons")["card_pm"]["PaymentMethod"];

      cy.createPaymentMethodTest(globalState, data);
    });

    it("create-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["PaymentIntentOffSession"];

      cy.createPaymentIntentTest(
        fixtures.createPaymentBody,
        data,
        "no_three_ds",
        "automatic",
        globalState
      );
      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("confirm-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["SaveCardUseNo3DSAutoCaptureOffSession"];

      cy.confirmCallTest(fixtures.confirmBody, data, true, globalState);
      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    it("Set default payment method", () => {
      cy.setDefaultPaymentMethodTest(globalState);
    });
  });

  context("Delete payment method for customer", () => {
    it("Create customer", () => {
      cy.createCustomerCallTest(fixtures.customerCreateBody, globalState);
    });

    it("Create Payment Method", () => {
      const data = getConnectorDetails("commons")["card_pm"]["PaymentMethod"];
      cy.createPaymentMethodTest(globalState, data);
    });

    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    it("Delete Payment Method for a customer", () => {
      cy.deletePaymentMethodTest(globalState);
    });
  });

  context.only("Last Used Token Payments", () => {
    let shouldContinue = true;

    beforeEach(function () {
      saveCardBody = Cypress._.cloneDeep(fixtures.saveCardConfirmBody);
      if (!shouldContinue) {
        this.skip();
      }
    });

    // Create a payment with one card (no 3ds)
    // setup future usage off_session with customer acceptance in confirm call
    // should get a token in confirm response
    it("create-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["PaymentIntentOffSession"];

      cy.createPaymentIntentTest(
        fixtures.createPaymentBody,
        data,
        "no_three_ds",
        "automatic",
        globalState
      );

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("confirm-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["SaveCardUseNo3DSAutoCaptureOffSession"];

      cy.confirmCallTest(fixtures.confirmBody, data, true, globalState);

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    // List payment method for a customer
    // token should be listed
    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    // Create a payment with another card (3ds)
    // setup future usage off_session with customer acceptance in confirm call
    // should get a token in confirm response
    it("create-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["PaymentIntentOffSession"];

      cy.createPaymentIntentTest(
        fixtures.createPaymentBody,
        data,
        "three_ds",
        "automatic",
        globalState
      );

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("confirm-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["SaveCardUse3DSAutoCaptureOffSession"];

      cy.confirmCallTest(fixtures.confirmBody, data, true, globalState);

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("Handle redirection", () => {
      const expectedRedirection = fixtures.confirmBody["return_url"];
      cy.handleRedirection(globalState, expectedRedirection);
    });

    // List payment method for a customer
    // token should be listed
    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    // Create a payment using token and off_session
    // confirm call off session with token

    it("create-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["PaymentIntent"];

      cy.createPaymentIntentTest(
        fixtures.createPaymentBody,
        data,
        "no_three_ds",
        "automatic",
        globalState
      );

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("confirm-save-card-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["SaveCardUseNo3DSAutoCapture"];

      cy.saveCardConfirmCallTest(saveCardBody, data, globalState);

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    // Now list payment method for same customer and verify the last used card should be at the top
    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });

    // Create a payment with token and on_session with another card which is at the bottom
    // confirm call on session with token

    it("create-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["PaymentIntent"];

      cy.createPaymentIntentTest(
        fixtures.createPaymentBody,
        data,
        "no_three_ds",
        "automatic",
        globalState
      );

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    it("confirm-save-card-payment-call-test", () => {
      const data = getConnectorDetails(globalState.get("connectorId"))[
        "card_pm"
      ]["SaveCardUseNo3DSAutoCapture"];

      cy.saveCardConfirmCallTest(saveCardBody, data, globalState);

      if (shouldContinue) shouldContinue = utils.should_continue_further(data);
    });

    // Now list payment method for same customer and verify the last used card should be at the top*
    it("List PM for customer", () => {
      cy.listCustomerPMCallTest(globalState);
    });
  });
});
