<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>smartcare-LOCATIONUPDATE - Automation</name>
   <tag></tag>
   <elementGuidId>25b5b337-21a8-4c95-9f22-a49d55c61394</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:v1\u003d\&quot;http://seq.huawei.com/interface/dsi/v1.1\&quot;\u003e\n\u003csoapenv:RequestHeader\u003e\n\t\u003creqTime\u003e2020-02-13 00:04:00\u003c/reqTime\u003e\n\t\u003c/soapenv:RequestHeader\u003e\n \u003csoapenv:RequestBody\u003e\n                   \u003cLocationUserList\u003e \n                           \u003cUser\u003e\n\t\t\t\t\t\t\u003cstarttime\u003e2020-06-30       \t\t05:05:45\u003c/starttime\u003e\n\u003cmsisdn\u003e${locupdatemsisdn}\u003c/msisdn\u003e\n\t\t\t\t\t\t\u003cimsi\u003e630010241502158\u003c/imsi\u003e\n\t\t\t\t\t\t\u003cimei\u003e357089104132958\u003c/imei\u003e\n\u003ccell_id\u003e${locupdatecellid}\u003c/cell_id\u003e\n\t\t\t\t\t\t\u003clayer2id\u003e8\u003c/layer2id\u003e\n\t\t\t\t\t\t\u003csrv_type\u003eLocation Update\u003c/srv_type\u003e\n\t\t\t\t\t\t\u003crat\u003e1\u003c/rat\u003e\n                        \u003cevent_id\u003e8004\u003c/event_id\u003e\n\t\t\t\t\t\t\u003cevent_type\u003eCEP_ypeijia_20200219145846417\u003c/event_type\u003e\n\t\t\t\t\t\t\u003c/User\u003e\n\t\t\t\t\t                 \u003c/LocationUserList\u003e            \n    \u003c/soapenv:RequestBody\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${smartcare_url}/SmartCarePlugin/WSDLLoader?group=LOCATIONUPDATE</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.locupdatemsisdn</defaultValue>
      <description></description>
      <id>2a0aed47-2ea6-49c9-af64-826914296929</id>
      <masked>false</masked>
      <name>locupdatemsisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.locupdatecellid</defaultValue>
      <description></description>
      <id>2f389fcd-c08f-42f9-adb3-5a356d7228be</id>
      <masked>false</masked>
      <name>locupdatecellid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.smartcare_url</defaultValue>
      <description></description>
      <id>471cb699-a16d-4736-aecb-98dbb1d7ef38</id>
      <masked>false</masked>
      <name>smartcare_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
