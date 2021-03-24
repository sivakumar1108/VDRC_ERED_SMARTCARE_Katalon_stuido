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

GlobalVariable.RAM_Bundle = '117'
GlobalVariable.RAM_SUBCOSID='500031'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '117'
GlobalVariable.RAM_SUBCOSID='500034'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '117'
GlobalVariable.RAM_SUBCOSID='500037'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '117'
GlobalVariable.RAM_SUBCOSID='500033'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))
WS.delay(30)

GlobalVariable.RAM_Bundle = '017'
GlobalVariable.RAM_SUBCOSID='500031'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '017'
GlobalVariable.RAM_SUBCOSID='500034'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '017'
GlobalVariable.RAM_SUBCOSID='500037'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '017'
GlobalVariable.RAM_SUBCOSID='500033'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))
WS.delay(30)
GlobalVariable.RAM_Bundle = '015'
GlobalVariable.RAM_SUBCOSID='500031'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '015'
GlobalVariable.RAM_SUBCOSID='500034'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '015'
GlobalVariable.RAM_SUBCOSID='500037'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '015'
GlobalVariable.RAM_SUBCOSID='500033'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))
WS.delay(30)
WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))
WS.delay(30)
GlobalVariable.RAM_Bundle = '115'
GlobalVariable.RAM_SUBCOSID='500031'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '115'
GlobalVariable.RAM_SUBCOSID='500034'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '115'
GlobalVariable.RAM_SUBCOSID='500037'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(30)
GlobalVariable.RAM_Bundle = '115'
GlobalVariable.RAM_SUBCOSID='500033'

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))
WS.delay(30)