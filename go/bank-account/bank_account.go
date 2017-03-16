package account

import "sync"

const testVersion = 1

type Account struct {
	mutex   sync.Mutex
	open    bool
	balance int64
}

func (acct *Account) Balance() (balance int64, ok bool) {
	balance, ok = acct.balance, false
	if acct.open {
		ok = true
	}
	return
}

func (acct *Account) Deposit(amount int64) (newBalance int64, ok bool) {
	acct.mutex.Lock()
	defer acct.mutex.Unlock()

	if acct.open && acct.balance+amount >= 0 {
		acct.balance += amount
		return acct.balance, true
	}
	return acct.balance, false
}

func (acct *Account) Close() (payout int64, ok bool) {
	acct.mutex.Lock()
	defer acct.mutex.Unlock()

	if acct.open {
		acct.open = false
		return acct.balance, true
	}
	return 0, false
}

func Open(initialDeposit int64) *Account {
	if initialDeposit < 0 {
		return nil
	}
	return &Account{open: true, balance: initialDeposit}
}
