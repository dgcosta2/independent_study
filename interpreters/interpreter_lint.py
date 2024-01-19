from ast import *
from utils import input_int, add64, sub64, neg64

class Interp_Lint:
    def interp_exp(self, e):
        match e:
            case BinOp(left, Add(), right):
                l = self.interp_exp(left); r = self.interp_exp(right)
                return add64(l, r)
            case BinOp(left, Sub(), right):
                l = self.interp_exp(left); r = self.interp_exp(right)
                return sub64(l, r)
            case UnaryOp(USub(), op):
                o = self.interp_exp(op)
                return neg64(o)
            case Call(Name('input_int'),[]):
                return input_int()
            case Constant(value):
                return value

    def interp_stmt(self, s):
        match s:
            case Expr(Call(Name('print'),[args])):
                return print(self.interp_exp(args))
            case Expr(arg):
                return self.interp_exp(arg)

    def interp_lint(self, p):
        match p:
            case Module(body):
                for s in body:
                    self.interp_stmt(s)

class Interp_Lvar(Interp_Lint):
    def interp_exp
def interp(p):
    return Interp_Lint().interp_lint(p)

if __name__ == "__main__":
    eight = Constant(8)
    neg_eight = UnaryOp(USub(), eight)
    read = Call(Name('input_int'), [])
    ast1_1 = BinOp(read, Add(), neg_eight)
    pr = Expr(Call(Name('print'), [ast1_1]))
    p = Module([pr])
    interp(p)
