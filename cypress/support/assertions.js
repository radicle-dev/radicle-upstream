const isInViewport = (chai, _utils) => {
  function assertIsInViewport(_options) {
    const subject = this._obj;

    const bottom = Cypress.$(cy.state("window")).height();
    const rect = subject[0].getBoundingClientRect();

    this.assert(
      rect.top < bottom && rect.bottom < bottom && rect.bottom > 0,
      "expected #{this} to be in viewport",
      "expected #{this} to not be in viewport",
      this._obj,
    );
  }

  chai.Assertion.addMethod("inViewport", assertIsInViewport);
};

chai.use(isInViewport);
