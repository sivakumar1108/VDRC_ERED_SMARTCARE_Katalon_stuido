import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

GlobalVariable.rag_msisdn = 123457661

GlobalVariable.rag_rech_amnt = '610000'

WS.sendRequest(findTestObject('010_Recharge_and_Get/Recharge_and_get'))

WS.delay(20)

GlobalVariable.rag_msisdn = 123457661

GlobalVariable.rag_rech_amnt = '310000'

WS.sendRequest(findTestObject('010_Recharge_and_Get/Recharge_and_get'))

WS.delay(20)

GlobalVariable.rag_msisdn = 123457661

GlobalVariable.rag_rech_amnt = '110000'

WS.sendRequest(findTestObject('010_Recharge_and_Get/Recharge_and_get'))

