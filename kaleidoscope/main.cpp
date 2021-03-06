#include <string>
#include <memory>
#include <vector>
#include <map>
// #include <llvm/ADT/APFloat.h>
// #include <llvm/ADT/STLExtras.h>
#include <llvm/IR/Value.h>
#include <llvm/IR/BasicBlock.h>
// #include <llvm/IR/Constant.h>
// #include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
// #include <llvm/IR/Module.h>
 #include <llvm/IR/Type.h>
#include <llvm/IR/Verifier.h>
//#include <llvm/Target/TargetMachine.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar.h>
#include <llvm/Transforms/Scalar/GVN.h>
#include "KaleidoscopeJIT.h"

//lexer只支持ascii，认识的返回负数，不认识的返回[0-255]
enum Token {
    tok_eof = -1,
    tok_def = -2,
    tok_extern = -3,
    tok_identifier = -4,
    tok_number = -5,
};

static std::string IdentifierStr;
static double NumVal;

//看起来只是《编程语言实现模式》说的“解析树”

static int gettok() {
    static int LastChar = ' ';
    while (isspace(LastChar))
        LastChar = getchar();
    //identifier: [a-zA-Z][a-zA-Z0-9]*
    if (isalpha(LastChar)) {
        IdentifierStr = LastChar;
        while (isalnum((LastChar = getchar())))
            IdentifierStr += LastChar;
        if (IdentifierStr == "def")
            return tok_def;
        if (IdentifierStr == "extern")
            return tok_extern;
        return tok_identifier;
    }
    //数字，只支持double，对于.12.34会识别错误
    if (isdigit(LastChar) || LastChar == '.') {
        std::string NumStr;
        do {
            NumStr += LastChar;
            LastChar = getchar();
        } while (isdigit(LastChar) || LastChar == '.');
        NumVal = strtod(NumStr.c_str(), 0);
        return tok_number;
    }
    //注释
    if (LastChar == '#') {
        do
            LastChar = getchar();
        while (LastChar != EOF && LastChar != '\n' && LastChar != '\r');
        if (LastChar != EOF)
            return gettok();
    }
    if (LastChar == EOF)
        return tok_eof;
    int ThisChar = LastChar;
    LastChar = getchar();
    return ThisChar;
}

class PrototypeAST;
static llvm::LLVMContext TheContext;
static llvm::IRBuilder<> Builder(TheContext);
static std::unique_ptr<llvm::Module> TheModule;
static std::map<std::string, llvm::Value*> NamedValues;
static std::unique_ptr<llvm::orc::KaleidoscopeJIT> TheJIT;
static std::unique_ptr<llvm::FunctionPassManager> TheFPM;



void InitializeModuleAndPassManager(void) {
    TheModule = std::make_unique<llvm::Module>("my cool jit", TheContext);
    TheModule->setDataLayout(TheJIT->getTargetMachine().createDataLayout());
    TheFPM = std::make_unique<llvm::FunctionPassManager>(TheModule.get());
    TheFPM->addPass(llvm::createInstructionCombiningPass());
    TheFPM->addPass(llvm::createReassociatePass());
    TheFPM->addPass(llvm::createGVNPass());
    TheFPM->addPass(llvm::createCFGSimplificationPass());
}

class ExprAST;
std::unique_ptr<ExprAST> LogError(const char* Str);

llvm::Value* LogErrorV(const char* Str) {
    LogError(Str);
    return nullptr;
}


class ExprAST {
public:
    virtual ~ExprAST() {}
    virtual llvm::Value* codegen() = 0;
};

class NumberExprAST : public ExprAST {
public:
    NumberExprAST(double Val) : Val(Val) {}
    virtual llvm::Value* codegen() override {
        return llvm::ConstantFP::get(TheContext, llvm::APFloat(Val));
    }
private:
    double Val;
};

class VariableExprAST : public ExprAST {
public:
    VariableExprAST(const std::string& Name) : Name(Name) {}
    virtual llvm::Value* codegen() {
        llvm::Value* V = NamedValues[Name];
        if (!V)
            LogErrorV("unknown variable name");
        return V;
    }
private:
    std::string Name;
};

class BinaryExprAST : public ExprAST {
public:
    BinaryExprAST(char op, std::unique_ptr<ExprAST> LHS,
            std::unique_ptr<ExprAST> RHS)
        : Op(op), LHS(std::move(LHS)), RHS(std::move(RHS)) {}
    virtual llvm::Value* codegen() {
        llvm::Value* L = LHS->codegen();
        llvm::Value* R = RHS->codegen();
        if (!L || !R)
            return nullptr;
        switch (Op) {
        case '+':
            return Builder.CreateFAdd(L, R, "addtmp");
        case '-':
            return Builder.CreateFSub(L, R, "subtmp");
        case '*':
            return Builder.CreateFMul(L, R, "multmp");
        case '<':
            L = Builder.CreateFCmpULT(L, R, "cmptmp");
            return Builder.CreateUIToFP(L, llvm::Type::getDoubleTy(TheContext), "booltmp");
        default:
            return LogErrorV("invalid binary operator");
        }
    }
private:
    char Op;
    std::unique_ptr<ExprAST> LHS;
    std::unique_ptr<ExprAST> RHS;
};

class CallExprAST : public ExprAST {
public:
    CallExprAST(const std::string& Callee,
            std::vector<std::unique_ptr<ExprAST>> Args)
        : Callee(Callee), Args(std::move(Args)) {}
    virtual llvm::Value* codegen() override {
        llvm::Function* CalleeF = TheModule->getFunction(Callee);
        if (!CalleeF)
            return LogErrorV("unknown function referenced");
        if (CalleeF->arg_size() != Args.size())
            return LogErrorV("incorrect # arguments passed");
        std::vector<llvm::Value*> ArgsV;
        for (uint i=0, e=Args.size(); i!=e; ++i) {
            ArgsV.push_back(Args[i]->codegen());
            if (!ArgsV.back())
                return nullptr;
        }
        return Builder.CreateCall(CalleeF, ArgsV, "calltmp");
    }
private:
    std::string Callee;
    std::vector<std::unique_ptr<ExprAST>> Args;
};

class PrototypeAST {
public:
    PrototypeAST(const std::string& Name, std::vector<std::string> Args)
        : Name(Name), Args(std::move(Args)) {}
    const std::string& getName() const { return Name; }
    llvm::Function* codegen() {
        std::vector<llvm::Type*> Doubles(Args.size(), llvm::Type::getDoubleTy(TheContext));
        llvm::FunctionType* FT = llvm::FunctionType::get(llvm::Type::getDoubleTy(TheContext), Doubles, false);
        llvm::Function* F = llvm::Function::Create(FT, llvm::Function::ExternalLinkage, Name, TheModule.get());
        uint Idx = 0;
        for (auto& Arg : F->args())
            Arg.setName(Args[Idx++]);
        return F;
    }
private:
    std::string Name;
    std::vector<std::string> Args;
};

static std::map<std::string, std::unique_ptr<PrototypeAST>> FunctionProtos;


llvm::Function* getFunction(std::string Name) {
    if (auto* F = TheModule->getFunction(Name))
        return F;
    auto FI = FunctionProtos.find(Name);
    if (FI != FunctionProtos.find(Name))
        return FI->second->codegen();
    return nullptr;
}

class FunctionAST {
public:
    FunctionAST(std::unique_ptr<PrototypeAST> Proto,
            std::unique_ptr<ExprAST> Body)
        : Proto(std::move(Proto)), Body(std::move(Body)) {}
    llvm::Function* codegen() {
        auto& P = *Proto;
        FunctionProtos[Proto->getName()] = std::move(Proto);
        llvm::Function* TheFunction = getFunction(P.getName());
        if (!TheFunction)
            return nullptr;

        llvm::BasicBlock* BB = llvm::BasicBlock::Create(TheContext, "entry", TheFunction);
        Builder.SetInsertPoint(BB);
        NamedValues.clear();
        //直接取地址？
        for (auto& Arg : TheFunction->args())
            NamedValues[Arg.getName()] = &Arg;
        if (llvm::Value* RetValue = Body->codegen()) {
            Builder.CreateRet(RetValue);
            llvm::verifyFunction(*TheFunction);
            TheFPM->run(*TheFunction);
            return TheFunction;
        }
        TheFunction->eraseFromParent();
        return nullptr;
    }
private:
    std::unique_ptr<PrototypeAST> Proto;
    std::unique_ptr<ExprAST> Body;
};


//trick，运算符优先级
static std::map<char, int> BinopPrecedence;
static int CurTok;
static int getNextToken() {
    return CurTok = gettok();
}


std::unique_ptr<ExprAST> LogError(const char* Str) {
    fprintf(stderr, "LogError: %s\n", Str);
    return nullptr;
}

std::unique_ptr<PrototypeAST> LogErrorP(const char* Str) {
    LogError(Str);
    return nullptr;
}

static std::unique_ptr<ExprAST> ParseExpression();

static std::unique_ptr<ExprAST> ParserNumberExpr() {
    auto Result = std::make_unique<NumberExprAST>(NumVal);
    getNextToken();
    return std::move(Result);
}

//括号并不需要展现在ast中
static std::unique_ptr<ExprAST> ParseParenExpr() {
    getNextToken();
    auto V = ParseExpression();
    if (!V)
        return nullptr;
    if (CurTok != ')')
        return LogError("expected ')'");
    getNextToken();
    return V;
}

//标识，或 函数调用
static std::unique_ptr<ExprAST> ParseIdentifierExpr() {
    std::string IdName = IdentifierStr;

    getNextToken();

    if (CurTok != '(')
        return std::make_unique<VariableExprAST>(IdName);
    
    getNextToken();
    std::vector<std::unique_ptr<ExprAST>> Args;
    if (CurTok != ')') {
        while (true) {
            if (auto Arg = ParseExpression())
                Args.push_back(std::move(Arg));
            else
                return nullptr;
            if (CurTok == ')')
                break;
            if (CurTok != ',')
                return LogError("Expected ')' or ',' in argument list");
            getNextToken();
        }
    }
    getNextToken();
    return std::make_unique<CallExprAST>(IdName, std::move(Args));
}

//标识（包括函数调用）、数字、括号
static std::unique_ptr<ExprAST> ParsePrimary() {
    switch (CurTok) {
    case tok_identifier:
        return ParseIdentifierExpr();
    case tok_number:
        return ParserNumberExpr();
    case '(':
        return ParseParenExpr();
    default:
        return LogError("unknown token when expecting an expression");
    }
}

//获取运算符优先级，由外面保证当前tok是运算符
static int GetTokPrecedence() {
    if (!isascii(CurTok))
        return -1;

    int TokPrec = BinopPrecedence[CurTok];
    if (TokPrec <= 0)
        return -1;
    return TokPrec;
}

static std::unique_ptr<ExprAST> ParseBinOpRHS(int ExprPrec, std::unique_ptr<ExprAST> LHS) {
    while (true) {
        int TokPrec = GetTokPrecedence();

        if (TokPrec < ExprPrec)
            return LHS;
        
        int BinOp = CurTok;
        getNextToken();

        auto RHS = ParsePrimary();
        if (!RHS)
            return nullptr;
        
        //wow
        int NextPrec = GetTokPrecedence();
        if (TokPrec < NextPrec) {
            RHS = ParseBinOpRHS(TokPrec + 1, std::move(RHS));
            if (!RHS)
                return nullptr;
        }

        LHS = std::make_unique<BinaryExprAST>(BinOp, std::move(LHS), std::move(RHS));
    }
}

//表达式一定是 lhs op rhs ??
static std::unique_ptr<ExprAST> ParseExpression() {
    auto LHS = ParsePrimary();
    if (!LHS)
        return nullptr;
    return ParseBinOpRHS(0, std::move(LHS));
}

static std::unique_ptr<PrototypeAST> ParsePrototype() {
    if (CurTok != tok_identifier)
        return LogErrorP("Expected function name in prototype");
    
    std::string FnName = IdentifierStr;
    getNextToken();

    if (CurTok != '(')
        return LogErrorP("Expected '(' in prototype");

    std::vector<std::string> ArgNames;
    //没有逗号
    while (getNextToken() == tok_identifier)
        ArgNames.push_back(IdentifierStr);
    if (CurTok != ')')
        return LogErrorP("Expected ')' in prototype");
    
    getNextToken();
    return std::make_unique<PrototypeAST>(FnName, std::move(ArgNames));
}

//单语句
static std::unique_ptr<FunctionAST> ParseDefinition() {
    getNextToken();
    auto Proto = ParsePrototype();
    if (!Proto)
        return nullptr;
    if (auto E = ParseExpression())
        return std::make_unique<FunctionAST>(std::move(Proto), std::move(E));
    return nullptr;
}

//这是啥
static std::unique_ptr<FunctionAST> ParseTopLevelExpr() {
    if (auto E = ParseExpression()) {
        auto Proto = std::make_unique<PrototypeAST>("__anon_expr", std::vector<std::string>());
        return std::make_unique<FunctionAST>(std::move(Proto), std::move(E));
    }
    return nullptr;
}

static std::unique_ptr<PrototypeAST> ParseExtern() {
    getNextToken();
    return ParsePrototype();
}

static void HandleDefinition() {
    if (auto FnAst = ParseDefinition()) {
        if (auto* FnIR = FnAst->codegen()) {
            fprintf(stderr, "Read function definition: \n");
            FnIR->print(llvm::errs());
            fprintf(stderr, "\n");
            TheJIT->addModule(std::move(TheModule));
            InitializeModuleAndPassManager();
        }
    } else {
        getNextToken();
    }
}

static void HandleExtern() {
    if (auto ProtoAST = ParseExtern()) {
        if (auto* FnIR = ProtoAST->codegen()) {
            fprintf(stderr, "Read extern: \n");
            FnIR->print(llvm::errs());
            fprintf(stderr, "\n");
        }
    } else {
        getNextToken();
    }
}

static void HandleTopLevelExpression() {
    if (auto FnAST = ParseTopLevelExpr()) {
        if (FnAST->codegen()) {
            auto H = TheJIT->addModule(std::move(TheModule));
            InitializeModuleAndPassManager();
            auto ExprSymbol = TheJIT->findSymbol("__anon_expr");
            assert(ExprSymbol && "Function Not Found");
            auto FP = (double(*)())(intptr_t)ExprSymbol.getAddress().get();
            fprintf(stderr, "Evaluated to %f\n", FP());
            TheJIT->removeModule(H);
        }
    } else {
        getNextToken();
    }
}

static void MainLoop() {
    while (true) {
        fprintf(stderr, "ready> ");
        switch (CurTok) {
        case tok_eof:
            return;
        case ';':   //??
            getNextToken();
            break;
        case tok_def:
            HandleDefinition();
            break;
        case tok_extern:
            HandleExtern();
            break;
        default:
            HandleTopLevelExpression();
            break;
        }
    }
}

int main() {
    llvm::InitializeNativeTarget();
    llvm::InitializeNativeTargetAsmPrinter();
    llvm::InitializeNativeTargetAsmParser();
    BinopPrecedence['<'] = 10;
    BinopPrecedence['+'] = 20;
    BinopPrecedence['-'] = 20;
    BinopPrecedence['*'] = 40;

    fprintf(stderr, "ready> ");
    getNextToken();
    //TheModule = std::make_unique<llvm::Module>("my cool jit", TheContext);
    TheJIT = std::make_unique<llvm::orc::KaleidoscopeJIT>();
    InitializeModuleAndPassManager();
    MainLoop();
    //TheModule->print(llvm::errs(), nullptr);
    return 0;
}